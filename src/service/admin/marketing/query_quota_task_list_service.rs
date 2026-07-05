use crate::model::dto::marketing::{
    QuotaTask, QueryQuotaTaskListRequest, QueryQuotaTaskListResponse,
};
use crate::model::entity::task::{QuotaContent, QuotaScope, Task, TaskType};
use crate::repository::task::{TaskFilter, TaskRepo};
use result::code_error::CodeError;
use result::error_code;

pub async fn query_quota_task_list(
    repo: &dyn TaskRepo,
    req: QueryQuotaTaskListRequest,
) -> Result<QueryQuotaTaskListResponse, anyhow::Error> {
    let page = req.page as i64;
    let size = req.size as i64;
    let status = req.status.map(|s| s as i16);
    let filter = TaskFilter {
        type_: TaskType::QUOTA.0 as i16,
        page,
        size,
        status,
        scope: None,
    };

    let (total, tasks) = repo.query_task_list(&filter).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            e.to_string(),
        ))
    })?;
    let list = tasks.iter().map(to_dto).collect();
    Ok(QueryQuotaTaskListResponse { total, list })
}

pub(crate) fn to_dto(t: &Task) -> QuotaTask {
    let scope: QuotaScope = t
        .scope
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(QuotaScope {
            subscribers: Vec::new(),
            is_active: None,
            start_time: 0,
            end_time: 0,
            recipients: Vec::new(),
        });
    let content: QuotaContent = t
        .content
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(QuotaContent {
            reset_traffic: false,
            days: None,
            gift_type: None,
            gift_value: None,
        });
    QuotaTask {
        id: t.id,
        subscribers: scope.subscribers,
        is_active: scope.is_active,
        start_time: scope.start_time,
        end_time: scope.end_time,
        reset_traffic: content.reset_traffic,
        days: content.days.unwrap_or(0) as u64,
        gift_type: content.gift_type.unwrap_or(0) as u8,
        gift_value: content.gift_value.unwrap_or(0) as u64,
        objects: scope.recipients,
        status: t.status as u8,
        total: t.total,
        current: t.current,
        errors: t.errors.clone().unwrap_or_default(),
        created_at: t.created_at,
        updated_at: t.updated_at,
    }
}
