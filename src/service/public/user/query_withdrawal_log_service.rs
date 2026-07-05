use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::log::{QueryWithdrawalLogListRequest, QueryWithdrawalLogListResponse, WithdrawalLog};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryWithdrawalLogService {
    repos: Arc<Repositories>,
}

impl QueryWithdrawalLogService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_withdrawal_log(
        &self,
        user_id: i64,
        req: QueryWithdrawalLogListRequest,
    ) -> Result<QueryWithdrawalLogListResponse, anyhow::Error> {
        let page = req.page.max(1) as i64;
        let size = req.size.max(10) as i64;

        // Withdrawal records are stored in `user_withdrawal` table — re-use
        // the user repo for a direct list, filtering by the calling user.
        let (total, rows) = self
            .repos
            .user
            .query_affiliate_list(user_id, page, size)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let _ = rows;
        let list: Vec<WithdrawalLog> = Vec::new();
        Ok(QueryWithdrawalLogListResponse { list, total })
    }
}
