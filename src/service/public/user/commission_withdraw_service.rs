use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::user::CommissionWithdrawRequest;
use crate::model::entity::log::COMMISSION_TYPE_WITHDRAW;
use crate::model::entity::user::Withdrawal;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;
use result::code_error::CodeError;
use result::error_code;

pub struct CommissionWithdrawService {
    repos: Arc<Repositories>,
}

impl CommissionWithdrawService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn commission_withdraw(
        &self,
        user_id: i64,
        req: CommissionWithdrawRequest,
    ) -> Result<Withdrawal, anyhow::Error> {
        let now = Utc::now().timestamp_millis();
        let order_no = format!("CW{}{}", now, user_id);

        let record = Withdrawal {
            id: 0,
            user_id,
            amount: req.amount,
            content: Some(req.content),
            status: 0,
            reason: String::new(),
            created_at: now,
            updated_at: now,
        };

        let created = self
            .repos
            .user
            .insert_withdrawal(&record)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_INSERT_ERROR,
                    e.to_string()
                ))
            })?;

        Telemetry::commission(
            &self.repos,
            user_id,
            COMMISSION_TYPE_WITHDRAW,
            req.amount,
            &order_no,
        )
        .await;

        Ok(created)
    }
}
