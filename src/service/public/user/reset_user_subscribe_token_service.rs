use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;
use uuid::Uuid;

use crate::model::dto::subscribe::ResetUserSubscribeTokenRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct ResetUserSubscribeTokenService {
    repos: Arc<Repositories>,
}

impl ResetUserSubscribeTokenService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn reset_user_subscribe_token(
        &self,
        _user_id: i64,
        req: ResetUserSubscribeTokenRequest,
    ) -> Result<String, anyhow::Error> {
        let mut s = self
            .repos
            .user
            .find_one_subscribe(req.user_subscribe_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        s.token = Uuid::new_v4().to_string();
        s.updated_at = Utc::now().timestamp_millis();

        self.repos.user.update_subscribe(&s).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string()
            ))
        })?;

        Ok(s.token)
    }
}
