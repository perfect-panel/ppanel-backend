use std::sync::Arc;

use crate::model::dto::user::GetUserLoginLogsRequest;
use crate::model::entity::log::LogType;
use crate::model::entity::log::SystemLog;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_user_login_logs(
    repos: &Arc<Repositories>,
    req: GetUserLoginLogsRequest,
) -> Result<(Vec<SystemLog>, i64), anyhow::Error> {
    repos
        .log
        .filter_logs(
            req.page as i64,
            req.size as i64,
            Some(LogType::LOGIN.0),
            None,
            Some(req.user_id),
            None,
        )
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })
}
