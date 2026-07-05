use crate::model::dto::marketing::{
    BatchSendEmailTask, GetBatchSendEmailTaskListRequest, GetBatchSendEmailTaskListResponse,
};
use crate::model::entity::task::{EmailContent, EmailScope, Task, TaskType};
use crate::repository::task::{TaskFilter, TaskRepo};
use result::code_error::CodeError;
use result::error_code;

pub async fn get_batch_send_email_task_list(
    repo: &dyn TaskRepo,
    req: GetBatchSendEmailTaskListRequest,
) -> Result<GetBatchSendEmailTaskListResponse, anyhow::Error> {
    let page = req.page as i64;
    let size = req.size as i64;
    let status = req.status.map(|s| s as i16);
    let filter = TaskFilter {
        type_: TaskType::EMAIL.0 as i16,
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
    Ok(GetBatchSendEmailTaskListResponse { total, list })
}

pub(crate) fn to_dto(t: &Task) -> BatchSendEmailTask {
    let scope: EmailScope = t
        .scope
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(EmailScope {
            type_: 0,
            register_start_time: 0,
            register_end_time: 0,
            recipients: Vec::new(),
            additional: Vec::new(),
            scheduled: 0,
            interval: 0,
            limit: 0,
        });
    let content: EmailContent = t
        .content
        .as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(EmailContent {
            subject: String::new(),
            content: String::new(),
        });
    BatchSendEmailTask {
        id: t.id,
        subject: content.subject,
        content: content.content,
        recipients: scope.recipients.join(","),
        scope: scope.type_ as i8,
        register_start_time: scope.register_start_time,
        register_end_time: scope.register_end_time,
        additional: scope.additional.join(","),
        scheduled: scope.scheduled,
        interval: scope.interval as u8,
        limit: scope.limit as u64,
        status: t.status as u8,
        errors: t.errors.clone().unwrap_or_default(),
        total: t.total as u64,
        current: t.current as u64,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }
}
