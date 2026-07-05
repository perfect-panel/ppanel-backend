use crate::model::dto::marketing::{
    QueryQuotaTaskPreCountRequest, QueryQuotaTaskPreCountResponse,
};
use crate::repository::user::{SubscribeFilter, UserRepo};
use result::code_error::CodeError;
use result::error_code;

pub async fn query_quota_task_pre_count(
    repo: &dyn UserRepo,
    req: QueryQuotaTaskPreCountRequest,
) -> Result<QueryQuotaTaskPreCountResponse, anyhow::Error> {
    let filter = SubscribeFilter {
        subscribers: req.subscribers,
        is_active: req.is_active,
        start_time: req.start_time,
        end_time: req.end_time,
    };
    let count = repo
        .count_subscribes_by_filter(&filter)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    Ok(QueryQuotaTaskPreCountResponse { count })
}
