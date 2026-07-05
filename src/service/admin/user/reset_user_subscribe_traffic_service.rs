use std::sync::Arc;

use chrono::Utc;

use crate::model::entity::log::RESET_SUBSCRIBE_TYPE_ADVANCE;
use crate::model::entity::user::UserSubscribe;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;
use result::code_error::CodeError;
use result::error_code;

/// Reset `download` and `upload` counters to zero for a single UserSubscribe
/// and emit a `RESET_SUBSCRIBE` business audit log.
pub async fn reset_user_subscribe_traffic(
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

    sub.download = 0;
    sub.upload = 0;
    sub.updated_at = Utc::now().timestamp_millis();

    let updated = repos.user.update_subscribe(&sub).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        ))
    })?;

    Telemetry::reset_subscribe(repos, updated.user_id, RESET_SUBSCRIBE_TYPE_ADVANCE, None).await;

    Ok(updated)
}
