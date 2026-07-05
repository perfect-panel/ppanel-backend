use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::auth::BindOAuthCallbackRequest;
use crate::model::entity::user::AuthMethods;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct BindOAuthCallbackService {
    repos: Arc<Repositories>,
}

impl BindOAuthCallbackService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn bind_o_auth_callback(
        &self,
        user_id: i64,
        req: BindOAuthCallbackRequest,
    ) -> Result<(), anyhow::Error> {
        let now = Utc::now().timestamp_millis();

        let identifier = req
            .callback
            .get("identifier")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let method = AuthMethods {
            id: 0,
            user_id,
            auth_type: req.method,
            auth_identifier: identifier,
            verified: true,
            created_at: now,
            updated_at: now,
        };

        self.repos
            .user
            .upsert_user_auth_method(&method)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_UPDATE_ERROR,
                    e.to_string()
                ))
            })?;

        Ok(())
    }
}
