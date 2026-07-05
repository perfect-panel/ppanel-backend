use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::user::UnsubscribeRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UnsubscribeService {
    repos: Arc<Repositories>,
}

impl UnsubscribeService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn unsubscribe(
        &self,
        _user_id: i64,
        req: UnsubscribeRequest,
    ) -> Result<(), anyhow::Error> {
        let mut s = self
            .repos
            .user
            .find_one_subscribe(req.id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        s.status = 0;
        s.finished_at = Some(Utc::now().timestamp_millis());
        s.updated_at = Utc::now().timestamp_millis();

        self.repos.user.update_subscribe(&s).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string()
            ))
        })?;

        Ok(())
    }
}
