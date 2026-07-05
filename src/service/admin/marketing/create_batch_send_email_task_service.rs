use anyhow::Context;
use chrono::Utc;

use crate::config::Config;
use crate::model::dto::marketing::{BatchSendEmailTask, CreateBatchSendEmailTaskRequest};
use crate::model::entity::task::{EmailContent, EmailScope, Task, TaskType};
use crate::queue::redis_url;
use crate::queue::types::SCHEDULED_BATCH_SEND_EMAIL;
use crate::repository::task::TaskRepo;
use result::code_error::CodeError;
use result::error_code;

const STATUS_PENDING: i16 = 0;

pub async fn create_batch_send_email_task(
    repo: &dyn TaskRepo,
    cfg: &Config,
    req: CreateBatchSendEmailTaskRequest,
) -> Result<BatchSendEmailTask, anyhow::Error> {
    let now = Utc::now().timestamp_millis();

    let scope = EmailScope {
        type_: req.scope as i16,
        register_start_time: req.register_start_time.unwrap_or(0),
        register_end_time: req.register_end_time.unwrap_or(0),
        recipients: Vec::new(),
        additional: Vec::new(),
        scheduled: req.scheduled.unwrap_or(0),
        interval: req.interval.unwrap_or(0) as i16,
        limit: req.limit.unwrap_or(0) as i64,
    };
    let content = EmailContent {
        subject: req.subject.clone(),
        content: req.content.clone(),
    };

    let entity = Task {
        id: 0,
        type_: TaskType::EMAIL.0 as i16,
        scope: Some(serde_json::to_string(&scope).unwrap_or_default()),
        content: Some(serde_json::to_string(&content).unwrap_or_default()),
        status: STATUS_PENDING,
        errors: None,
        total: 0,
        current: 0,
        created_at: now,
        updated_at: now,
    };

    let saved = repo
        .insert(&entity)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                e.to_string(),
            ))
        })?;

    if let Err(e) = enqueue_batch_send_email(cfg, saved.id).await {
        tracing::warn!("[create_batch_send_email_task] enqueue failed: {e}");
    }

    Ok(super::get_batch_send_email_task_list_service::to_dto(&saved))
}

async fn enqueue_batch_send_email(cfg: &Config, task_id: i64) -> anyhow::Result<()> {
    let payload = serde_json::json!({ "id": task_id }).to_string();
    let url = redis_url(&cfg.redis);
    let redis_cfg =
        asynq::backend::RedisConnectionType::single(url).context("build redis connection")?;
    let client = asynq::client::Client::new(redis_cfg)
        .await
        .context("build asynq client")?;
    let task = asynq::task::Task::new(SCHEDULED_BATCH_SEND_EMAIL, payload.as_bytes())
        .context("build asynq task")?;
    client.enqueue(task).await.context("enqueue task")?;
    Ok(())
}

