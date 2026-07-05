use crate::model::dto::log::{
    FilterResetSubscribeLogRequest, FilterResetSubscribeLogResponse, ResetSubscribeLog,
};
use crate::model::entity::log::{LogType, ResetSubscribe};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_reset_subscribe_log(
    repo: &dyn LogRepo,
    req: FilterResetSubscribeLogRequest,
) -> Result<FilterResetSubscribeLogResponse, anyhow::Error> {
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::RESET_SUBSCRIBE.0),
            req.params.date.as_deref(),
            req.user_subscribe_id,
            req.params.search.as_deref(),
        )
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let list = rows
        .into_iter()
        .map(|row| {
            let content: ResetSubscribe = serde_json::from_str(&row.content).unwrap_or(ResetSubscribe {
                type_: 0,
                user_id: row.object_id,
                order_no: None,
                timestamp: row.created_at,
            });
            ResetSubscribeLog {
                type_: content.type_ as u16,
                user_id: content.user_id,
                user_subscribe_id: req.user_subscribe_id.unwrap_or(content.user_id),
                order_no: content.order_no,
                timestamp: content.timestamp,
            }
        })
        .collect();

    Ok(FilterResetSubscribeLogResponse { total, list })
}
