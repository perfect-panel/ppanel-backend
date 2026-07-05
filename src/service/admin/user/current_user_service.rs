use std::sync::Arc;

use crate::model::entity::user::User;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn current_user(
    repos: &Arc<Repositories>,
    user_id: i64,
) -> Result<User, anyhow::Error> {
    repos.user.find_one_user(user_id).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        ))
    })
}
