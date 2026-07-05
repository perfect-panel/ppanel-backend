use std::sync::Arc;

use chrono::Utc;

use crate::model::entity::user::UserSubscribe;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

/// Update a UserSubscribe. Caller passes a fully-formed `UserSubscribe` (typically
/// obtained via `find_one_subscribe` and then mutated). Only `updated_at` is
/// refreshed here.
pub async fn update_user_subscribe(
    repos: &Arc<Repositories>,
    mut sub: UserSubscribe,
) -> Result<UserSubscribe, anyhow::Error> {
    sub.updated_at = Utc::now().timestamp_millis();
    repos.user.update_subscribe(&sub).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        ))
    })
}
