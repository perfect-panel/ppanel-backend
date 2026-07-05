use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::auth::BindTelegramResponse;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct BindTelegramService {
    repos: Arc<Repositories>,
}

impl BindTelegramService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn bind_telegram(
        &self,
        user_id: i64,
    ) -> Result<BindTelegramResponse, anyhow::Error> {
        // TODO: build Telegram bot deep-link with HMAC-signed payload.
        let _ = self
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
        Ok(BindTelegramResponse {
            url: String::new(),
            expired_at: 0,
        })
    }
}
