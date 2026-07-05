use chrono::Utc;

use crate::model::dto::marketing::{CreateQuotaTaskRequest, QuotaTask};
use crate::model::entity::task::{QuotaContent, QuotaScope, Task, TaskType};
use crate::repository::task::TaskRepo;
use result::code_error::CodeError;
use result::error_code;

const STATUS_PENDING: i16 = 0;

pub async fn create_quota_task(
    repo: &dyn TaskRepo,
    req: CreateQuotaTaskRequest,
) -> Result<QuotaTask, anyhow::Error> {
    let now = Utc::now().timestamp_millis();

    let scope = QuotaScope {
        subscribers: req.subscribers.clone(),
        is_active: req.is_active,
        start_time: req.start_time,
        end_time: req.end_time,
        recipients: Vec::new(),
    };
    let content = QuotaContent {
        reset_traffic: req.reset_traffic,
        days: Some(req.days as i64),
        gift_type: Some(req.gift_type as i16),
        gift_value: Some(req.gift_value as i64),
    };

    let entity = Task {
        id: 0,
        type_: TaskType::QUOTA.0 as i16,
        scope: Some(serde_json::to_string(&scope).unwrap_or_default()),
        content: Some(serde_json::to_string(&content).unwrap_or_default()),
        status: STATUS_PENDING,
        errors: None,
        total: 0,
        current: 0,
        created_at: now,
        updated_at: now,
    };

    let saved = repo.insert(&entity).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            e.to_string(),
        ))
    })?;

    Ok(super::query_quota_task_list_service::to_dto(&saved))
}
