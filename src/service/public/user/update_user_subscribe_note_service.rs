use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::subscribe::UpdateUserSubscribeNoteRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UpdateUserSubscribeNoteService {
    repos: Arc<Repositories>,
}

impl UpdateUserSubscribeNoteService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn update_user_subscribe_note(
        &self,
        _user_id: i64,
        req: UpdateUserSubscribeNoteRequest,
    ) -> Result<(), anyhow::Error> {
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

        s.note = req.note;
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
