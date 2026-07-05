use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::auth::UnbindOAuthRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UnbindOAuthService {
    repos: Arc<Repositories>,
}

impl UnbindOAuthService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn unbind_o_auth(
        &self,
        user_id: i64,
        req: UnbindOAuthRequest,
    ) -> Result<(), anyhow::Error> {
        self.repos
            .user
            .delete_user_auth_methods(user_id, &req.method)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_DELETED_ERROR,
                    e.to_string()
                ))
            })?;

        Ok(())
    }
}
