use std::sync::Arc;

use crate::cache::Cache;
use crate::model::dto::user::KickOfflineRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

/// Force a device offline by deleting its session key from Redis.
///
/// The Go backend keyed active sessions as
///   `auth:session_id:<user_id>:<identifier>`
/// (see `config.cache_key.SESSION_ID_KEY`); clearing it terminates the next
/// auth_middleware check.
pub async fn kick_offline_by_user_device(
    repos: &Arc<Repositories>,
    cache: &Cache,
    req: KickOfflineRequest,
) -> Result<(), anyhow::Error> {
    // Resolve the device so we know the user_id + identifier.
    let device = repos
        .user
        .find_one_device(req.id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let key = format!(
        "{}:{}:{}",
        crate::config::cache_key::SESSION_ID_KEY,
        device.user_id,
        device.identifier
    );

    cache.del(&key).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_msg(&e.to_string()))
    })
}
