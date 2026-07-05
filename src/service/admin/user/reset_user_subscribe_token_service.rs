use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::model::entity::user::UserSubscribe;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

/// Mint a fresh token + uuid for a UserSubscribe. The old token is invalidated
/// implicitly because the row is updated.
pub async fn reset_user_subscribe_token(
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

    sub.token = Uuid::new_v4().to_string().replace('-', "");
    sub.uuid = Uuid::new_v4().to_string();
    sub.updated_at = Utc::now().timestamp_millis();

    repos.user.update_subscribe(&sub).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        ))
    })
}
