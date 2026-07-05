use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::auth::UserLoginLog;
use crate::model::dto::log::GetLoginLogRequest;
use crate::model::dto::log::GetLoginLogResponse;
use crate::model::entity::log::LogType;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetLoginLogService {
    repos: Arc<Repositories>,
}

impl GetLoginLogService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get_login_log(
        &self,
        user_id: i64,
        req: GetLoginLogRequest,
    ) -> Result<GetLoginLogResponse, anyhow::Error> {
        let page = req.page.max(1) as i64;
        let size = req.size.max(10) as i64;

        let (rows, total) = self
            .repos
            .log
            .filter_logs(
                page,
                size,
                Some(LogType::LOGIN.0),
                None,
                Some(user_id),
                None,
            )
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let list = rows
            .into_iter()
            .filter_map(|row| {
                let login: crate::model::entity::log::Login =
                    serde_json::from_str(&row.content).ok()?;
                Some(UserLoginLog {
                    id: row.id,
                    user_id: row.object_id,
                    login_ip: login.login_ip,
                    user_agent: login.user_agent,
                    success: login.success,
                    timestamp: login.timestamp,
                })
            })
            .collect();

        Ok(GetLoginLogResponse { list, total })
    }
}
