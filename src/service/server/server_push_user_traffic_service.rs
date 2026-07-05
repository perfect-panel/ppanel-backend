/// Server push user traffic service.
/// Ported from `server/internal/logic/server/serverPushUserTrafficLogic.go`.

use std::sync::Arc;

use anyhow::anyhow;

use crate::config::Config;
use crate::model::dto::server::ServerPushUserTrafficRequest;
use crate::repository::Repositories;
use crate::queue::types::FORTHWITH_TRAFFIC_STATISTICS;
use result::code_error::CodeError;
use result::error_code;

/// Accepts a traffic report from a node and enqueues a traffic-statistics task.
pub async fn server_push_user_traffic(
    repos: Arc<Repositories>,
    config: Arc<Config>,
    req: ServerPushUserTrafficRequest,
) -> Result<(), anyhow::Error> {
    let server_id = req.common.server_id;

    // Verify server exists
    let mut server = repos
        .node
        .find_one_server(server_id)
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    // Build the traffic-statistics payload (mirrors Go task.TrafficStatistics)
    let payload = serde_json::json!({
        "server_id": server.id,
        "protocol": req.common.protocol,
        "logs": req.traffic,
    });

    let payload_bytes = serde_json::to_vec(&payload)
        .map_err(|e| anyhow!("json encode traffic payload: {e}"))?;

    // Build asynq client and enqueue — mirrors queue/service/subscription.rs pattern
    let redis_url = make_redis_url(&config);
    match asynq::backend::RedisConnectionType::single(redis_url) {
        Ok(redis_cfg) => match asynq::client::Client::new(redis_cfg).await {
            Ok(client) => {
                match asynq::task::Task::new(FORTHWITH_TRAFFIC_STATISTICS, &payload_bytes) {
                    Ok(task) => {
                        if let Err(e) = client.enqueue(task).await {
                            tracing::error!("[ServerPushUserTraffic] enqueue error: {e}");
                        }
                    }
                    Err(e) => tracing::error!("[ServerPushUserTraffic] Task::new error: {e}"),
                }
            }
            Err(e) => tracing::error!("[ServerPushUserTraffic] Client::new error: {e}"),
        },
        Err(e) => tracing::error!("[ServerPushUserTraffic] redis cfg error: {e}"),
    }

    // Update last_reported_at in DB (best-effort)
    let now = chrono::Utc::now().timestamp();
    server.last_reported_at = Some(now);
    if let Err(e) = repos.node.update_server(&server).await {
        tracing::error!("[ServerPushUserTraffic] update_server error: {e}");
    }

    Ok(())
}

fn make_redis_url(config: &Config) -> String {
    let db = config.redis.db;
    if config.redis.pass.is_empty() {
        format!("redis://{}/{}", config.redis.host, db)
    } else {
        format!("redis://:{}@{}/{}", config.redis.pass, config.redis.host, db)
    }
}
