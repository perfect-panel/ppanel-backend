use crate::model::dto::log::{
    FilterLoginLogRequest, FilterLoginLogResponse, LoginLog,
};
use crate::model::entity::log::{Login, LogType};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_login_log(
    repo: &dyn LogRepo,
    req: FilterLoginLogRequest,
) -> Result<FilterLoginLogResponse, anyhow::Error> {
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::LOGIN.0),
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
            let content: Login = serde_json::from_str(&row.content).unwrap_or(Login {
                method: String::new(),
                login_ip: String::new(),
                user_agent: String::new(),
                success: false,
                timestamp: row.created_at,
            });
            LoginLog {
                user_id: row.object_id,
                method: content.method,
                login_ip: content.login_ip,
                user_agent: content.user_agent,
                success: content.success,
                timestamp: content.timestamp,
            }
        })
        .collect();

    Ok(FilterLoginLogResponse { total, list })
}
