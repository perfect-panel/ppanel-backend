use std::sync::Arc;

use crate::repository::user::SubscribeDetails;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_user_subscribe_by_id(
    repos: &Arc<Repositories>,
    id: i64,
) -> Result<SubscribeDetails, anyhow::Error> {
    repos
        .user
        .find_one_subscribe_details_by_id(id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })
}
