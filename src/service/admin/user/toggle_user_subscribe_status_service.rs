use std::sync::Arc;

use chrono::Utc;

use crate::model::entity::user::UserSubscribe;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

/// Toggle the `status` bit on a UserSubscribe (0 ↔ 1).
pub async fn toggle_user_subscribe_status(
    repos: &Arc<Repositories>,
    subscribe_id: i64,
) -> Result<UserSubscribe, anyhow::Error> {
    let mut sub = repos
        .user
        .find_one_subscribe(subscribe_id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    sub.status = if sub.status == 0 { 1 } else { 0 };
    sub.updated_at = Utc::now().timestamp_millis();

    repos.user.update_subscribe(&sub).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        ))
    })
}
