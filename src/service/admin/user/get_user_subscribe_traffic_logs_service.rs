use std::sync::Arc;

use crate::model::entity::log::LogType;
use crate::model::entity::log::SystemLog;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_user_subscribe_traffic_logs(
    repos: &Arc<Repositories>,
    user_id: i64,
    page: i32,
    size: i32,
) -> Result<(Vec<SystemLog>, i64), anyhow::Error> {
    repos
        .log
        .filter_logs(
            page as i64,
            size as i64,
            Some(LogType::SUBSCRIBE_TRAFFIC.0),
            None,
            Some(user_id),
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
