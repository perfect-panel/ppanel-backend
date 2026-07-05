use crate::model::dto::marketing::{
    QueryQuotaTaskStatusRequest, QueryQuotaTaskStatusResponse,
};
use crate::model::entity::task::TaskType;
use crate::repository::task::TaskRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn query_quota_task_status(
    repo: &dyn TaskRepo,
    req: QueryQuotaTaskStatusRequest,
) -> Result<QueryQuotaTaskStatusResponse, anyhow::Error> {
    let t = repo
        .find_one_by_type(req.id, TaskType::QUOTA.0 as i16)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    Ok(QueryQuotaTaskStatusResponse {
        status: t.status as u8,
        current: t.current,
        total: t.total,
        errors: t.errors.unwrap_or_default(),
    })
}
