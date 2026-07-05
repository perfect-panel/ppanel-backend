use std::sync::Arc;
use crate::cache::Cache;
use crate::config::Config;
use crate::model::dto::server::ServerPushStatusRequest;
use crate::repository::Repositories;

pub async fn server_push_status(
    repos: Arc<Repositories>,
    _config: Arc<Config>,
    _cache: Arc<Cache>,
    req: ServerPushStatusRequest,
) -> anyhow::Result<()> {
    let node = repos.node.find_one_node(req.common.server_id).await?;
    let mut updated = node;
    updated.updated_at = chrono::Utc::now().timestamp_millis();
    repos.node.update_node(&updated).await?;
    tracing::debug!(server_id = req.common.server_id, cpu = req.cpu, mem = req.mem, "node push status");
    Ok(())
}
