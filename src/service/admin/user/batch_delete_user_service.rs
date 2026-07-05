use std::sync::Arc;

use crate::model::dto::user::BatchDeleteUserRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn batch_delete_user(
    repos: &Arc<Repositories>,
    req: BatchDeleteUserRequest,
) -> Result<u64, anyhow::Error> {
    if req.ids.is_empty() {
        return Ok(0);
    }
    repos
        .user
        .batch_delete_users(&req.ids)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_DELETED_ERROR,
                &e.to_string(),
            ))
        })
}
