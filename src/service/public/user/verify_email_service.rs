use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::auth::VerifyEmailRequest;
use crate::model::entity::user::AuthMethods;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct VerifyEmailService {
    repos: Arc<Repositories>,
}

impl VerifyEmailService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn verify_email(
        &self,
        user_id: i64,
        req: VerifyEmailRequest,
    ) -> Result<(), anyhow::Error> {
        let now = Utc::now().timestamp_millis();

        let method = AuthMethods {
            id: 0,
            user_id,
            auth_type: "email".to_string(),
            auth_identifier: req.email.clone(),
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
