use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::user::UpdateUserPasswordRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UpdateUserPasswordService {
    repos: Arc<Repositories>,
}

impl UpdateUserPasswordService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn update_user_password(
        &self,
        user_id: i64,
        req: UpdateUserPasswordRequest,
    ) -> Result<(), anyhow::Error> {
        let u = self
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

        if !password::multi_password_verify(
            &u.algo,
            u.salt.as_deref().unwrap_or(""),
            &req.password,
            &u.password,
        ) {
            return Err(anyhow!(CodeError::new_err_code(error_code::USER_PASSWORD_ERROR)));
        }

        let new_hash = password::encode_password(&req.password).map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::ERROR,
                e.to_string()
            ))
        })?;

        let mut updated = u;
        updated.password = new_hash;
        updated.algo = "default".to_string();
        updated.salt = None;
        updated.updated_at = Utc::now().timestamp_millis();

        self.repos.user.update_user(&updated).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string()
            ))
        })?;

        Ok(())
    }
}
