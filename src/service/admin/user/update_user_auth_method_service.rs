use std::sync::Arc;

use chrono::Utc;

use crate::model::dto::user::UpdateUserAuthMethodRequest;
use crate::model::entity::user::AuthMethods;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_user_auth_method(
    repos: &Arc<Repositories>,
    req: UpdateUserAuthMethodRequest,
) -> Result<AuthMethods, anyhow::Error> {
    // Look up the existing row by (user_id, auth_type) and replace identifier.
    let existing = repos
        .user
        .find_auth_method_by_platform(req.user_id, &req.auth_type)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?
        .ok_or_else(|| {
            anyhow::Error::new(CodeError::new_err_code(error_code::USER_NOT_EXIST))
        })?;

    let mut updated = existing;
    updated.auth_identifier = req.auth_identifier;
    updated.updated_at = Utc::now().timestamp_millis();

    repos
        .user
        .update_auth_method(&updated)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                &e.to_string(),
            ))
        })
}
