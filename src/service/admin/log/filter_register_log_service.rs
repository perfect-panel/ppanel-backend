use crate::model::dto::log::{
    FilterRegisterLogRequest, FilterRegisterLogResponse, RegisterLog,
};
use crate::model::entity::log::{LogType, Register};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_register_log(
    repo: &dyn LogRepo,
    req: FilterRegisterLogRequest,
) -> Result<FilterRegisterLogResponse, anyhow::Error> {
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::REGISTER.0),
            req.params.date.as_deref(),
            req.user_id,
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
            let content: Register = serde_json::from_str(&row.content).unwrap_or(Register {
                auth_method: String::new(),
                identifier: String::new(),
                register_ip: String::new(),
                user_agent: String::new(),
                timestamp: row.created_at,
            });
            RegisterLog {
                user_id: row.object_id,
                auth_method: content.auth_method,
                identifier: content.identifier,
                register_ip: content.register_ip,
                user_agent: content.user_agent,
                timestamp: content.timestamp,
            }
        })
        .collect();

    Ok(FilterRegisterLogResponse { total, list })
}
