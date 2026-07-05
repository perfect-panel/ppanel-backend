use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UnbindTelegramService {
    repos: Arc<Repositories>,
}

impl UnbindTelegramService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn unbind_telegram(&self, user_id: i64) -> Result<(), anyhow::Error> {
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

        u.avatar = String::new();
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
