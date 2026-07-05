use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::user::UpdateBindEmailRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UpdateBindEmailService {
    repos: Arc<Repositories>,
}

impl UpdateBindEmailService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn update_bind_email(
        &self,
        user_id: i64,
        req: UpdateBindEmailRequest,
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

        u.avatar = req.email;
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
