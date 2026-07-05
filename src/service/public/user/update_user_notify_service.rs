use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::user::UpdateUserNotifyRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UpdateUserNotifyService {
    repos: Arc<Repositories>,
}

impl UpdateUserNotifyService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn update_user_notify(
        &self,
        user_id: i64,
        req: UpdateUserNotifyRequest,
    ) -> Result<(), anyhow::Error> {
        let mut u = self
            .repos
            .user
            .find_one_user(user_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        if let Some(v) = req.enable_balance_notify {
            u.enable_balance_notify = v;
        }
        if let Some(v) = req.enable_login_notify {
            u.enable_login_notify = v;
        }
        if let Some(v) = req.enable_subscribe_notify {
            u.enable_subscribe_notify = v;
        }
        if let Some(v) = req.enable_trade_notify {
            u.enable_trade_notify = v;
        }
        u.updated_at = Utc::now().timestamp_millis();

        self.repos.user.update_user(&u).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string()
            ))
        })?;

        Ok(())
    }
}
