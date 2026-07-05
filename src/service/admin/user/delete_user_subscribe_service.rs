use std::sync::Arc;

use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn delete_user_subscribe(
    repos: &Arc<Repositories>,
    subscribe_id: i64,
) -> Result<u64, anyhow::Error> {
    repos
        .user
        .delete_subscribe_by_id(subscribe_id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_DELETED_ERROR,
                &e.to_string(),
            ))
        })
}
