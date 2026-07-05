use anyhow::Context;
use crate::model::dto::auth::TestSmsSendRequest;
use crate::queue::types::FORTHWITH_SEND_SMS;
use crate::queue::redis_url;

pub async fn test_sms_send(
    cfg: &crate::config::Config,
    req: TestSmsSendRequest,
) -> anyhow::Result<()> {
    let payload = serde_json::json!({
        "area_code": req.area_code,
        "telephone": req.telephone,
        "code": "123456",
    });
    let payload_bytes = serde_json::to_vec(&payload).context("serialize sms payload")?;
    let url = redis_url(&cfg.redis);
    let redis_cfg = asynq::backend::RedisConnectionType::single(url)
        .context("build redis connection")?;
    let client = asynq::client::Client::new(redis_cfg).await.context("build asynq client")?;
    let task = asynq::task::Task::new(FORTHWITH_SEND_SMS, &payload_bytes)
        .context("build asynq task")?;
    client.enqueue(task).await.context("enqueue test sms")?;
    Ok(())
}
