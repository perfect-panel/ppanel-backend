use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::log::{FilterSubscribeLogRequest, FilterSubscribeLogResponse, SubscribeLog};
use crate::model::entity::log::LogType;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetSubscribeLogService {
    repos: Arc<Repositories>,
}

impl GetSubscribeLogService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get_subscribe_log(
        &self,
        user_id: i64,
        req: FilterSubscribeLogRequest,
    ) -> Result<FilterSubscribeLogResponse, anyhow::Error> {
        let page = req.params.page.max(1) as i64;
        let size = req.params.size.max(10) as i64;

        let (rows, total) = self
            .repos
            .log
            .filter_logs(
                page,
                size,
                Some(LogType::SUBSCRIBE.0),
                req.params.date.as_deref(),
                Some(user_id),
                req.params.search.as_deref(),
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
                let inner: crate::model::entity::log::SubscribeLog =
                    serde_json::from_str(&row.content).ok()?;
                Some(SubscribeLog {
                    user_id: row.object_id,
                    token: inner.token,
                    user_agent: inner.user_agent,
                    client_ip: inner.client_ip,
                    user_subscribe_id: inner.user_subscribe_id,
                    timestamp: row.created_at,
                })
            })
            .collect();

        Ok(FilterSubscribeLogResponse { list, total })
    }
}
