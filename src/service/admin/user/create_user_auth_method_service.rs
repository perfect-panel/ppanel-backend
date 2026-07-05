use std::sync::Arc;

use chrono::Utc;

use crate::model::dto::user::CreateUserAuthMethodRequest;
use crate::model::entity::user::AuthMethods;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_user_auth_method(
    repos: &Arc<Repositories>,
    req: CreateUserAuthMethodRequest,
) -> Result<AuthMethods, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let entity = AuthMethods {
        id: 0,
        user_id: req.user_id,
        auth_type: req.auth_type,
        auth_identifier: req.auth_identifier,
        verified: true,
        created_at: now,
        updated_at: now,
    };

    repos
        .user
        .insert_auth_method(&entity)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            ))
        })
}
