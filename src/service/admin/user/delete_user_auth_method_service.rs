use std::sync::Arc;

use crate::model::dto::user::DeleteUserAuthMethodRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn delete_user_auth_method(
    repos: &Arc<Repositories>,
    req: DeleteUserAuthMethodRequest,
) -> Result<u64, anyhow::Error> {
    repos
        .user
        .delete_user_auth_methods(req.user_id, &req.auth_type)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_DELETED_ERROR,
                &e.to_string(),
            ))
        })
}
