use std::sync::Arc;

use crate::model::dto::user::{GetUserAuthMethodRequest, GetUserAuthMethodResponse, UserAuthMethod};
use crate::model::entity::user::AuthMethods;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_user_auth_method(
    repos: &Arc<Repositories>,
    req: GetUserAuthMethodRequest,
) -> Result<GetUserAuthMethodResponse, anyhow::Error> {
    let methods: Vec<AuthMethods> = repos
        .user
        .find_user_auth_methods(req.user_id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let auth_methods = methods
        .into_iter()
        .map(|m| UserAuthMethod {
            auth_type: m.auth_type,
            auth_identifier: m.auth_identifier,
            verified: m.verified,
        })
        .collect();

    Ok(GetUserAuthMethodResponse { auth_methods })
}
