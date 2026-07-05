use crate::model::dto::log::{
    FilterSubscribeLogRequest, FilterSubscribeLogResponse, SubscribeLog,
};
use crate::model::entity::log::{LogType, SubscribeLog as SubscribeLogEntity};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_subscribe_log(
    repo: &dyn LogRepo,
    req: FilterSubscribeLogRequest,
) -> Result<FilterSubscribeLogResponse, anyhow::Error> {
    let object_id = req.user_subscribe_id.or(req.user_id);
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::SUBSCRIBE.0),
            req.params.date.as_deref(),
            object_id,
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
            let content: SubscribeLogEntity = serde_json::from_str(&row.content).unwrap_or(SubscribeLogEntity {
                token: String::new(),
                user_agent: String::new(),
                client_ip: String::new(),
                user_subscribe_id: row.object_id,
            });
            SubscribeLog {
                user_id: row.object_id,
                token: content.token,
                user_agent: content.user_agent,
                client_ip: content.client_ip,
                user_subscribe_id: content.user_subscribe_id,
                timestamp: row.created_at,
            }
        })
        .collect();

    Ok(FilterSubscribeLogResponse { total, list })
}
