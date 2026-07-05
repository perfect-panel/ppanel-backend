use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::model::entity::user::UserSubscribe;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

/// Create a new UserSubscribe for a user.
///
/// NOTE: This is a minimal skeleton. Production logic (purchase vs admin-grant,
/// product/duration expansion, gift quota, order linkage, status bitmask) is
/// TODO and should be ported from `server/internal/logic/admin/user/createUserSubscribeLogic.go`.
pub async fn create_user_subscribe(
    repos: &Arc<Repositories>,
    user_id: i64,
    subscribe_id: i64,
    duration_days: i64,
    traffic: i64,
) -> Result<UserSubscribe, anyhow::Error> {
    let _ = (repos, user_id, subscribe_id, duration_days, traffic);
    let now = Utc::now().timestamp_millis();
    let entity = UserSubscribe {
        id: 0,
        user_id,
        order_id: 0,
        subscribe_id,
        start_time: now,
        expire_time: now + duration_days * 86_400_000,
        finished_at: None,
        traffic,
        download: 0,
        upload: 0,
        token: Uuid::new_v4().to_string().replace('-', ""),
        uuid: Uuid::new_v4().to_string(),
        status: 1,
        note: String::new(),
        created_at: now,
        updated_at: now,
    };

    repos
        .user
        .insert_subscribe(&entity)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            ))
        })
}
