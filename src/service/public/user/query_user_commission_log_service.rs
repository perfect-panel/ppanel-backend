use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::log::{CommissionLog, FilterCommissionLogRequest, FilterCommissionLogResponse};
use crate::model::entity::log::LogType;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryUserCommissionLogService {
    repos: Arc<Repositories>,
}

impl QueryUserCommissionLogService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_user_commission_log(
        &self,
        user_id: i64,
        req: FilterCommissionLogRequest,
    ) -> Result<FilterCommissionLogResponse, anyhow::Error> {
        let page = req.params.page.max(1) as i64;
        let size = req.params.size.max(10) as i64;

        let (rows, total) = self
            .repos
            .log
            .filter_logs(
                page,
                size,
                Some(LogType::COMMISSION.0),
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
                let inner: crate::model::entity::log::Commission =
                    serde_json::from_str(&row.content).ok()?;
                Some(CommissionLog {
                    type_: inner.type_ as u16,
                    user_id: row.object_id,
                    amount: inner.amount,
                    order_no: inner.order_no,
                    timestamp: inner.timestamp,
                })
            })
            .collect();

        Ok(FilterCommissionLogResponse { list, total })
    }
}
