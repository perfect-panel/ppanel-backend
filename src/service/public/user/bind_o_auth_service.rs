use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::auth::BindOAuthRequest;
use crate::model::dto::auth::BindOAuthResponse;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct BindOAuthService {
    repos: Arc<Repositories>,
}

impl BindOAuthService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn bind_o_auth(
        &self,
        _user_id: i64,
        req: BindOAuthRequest,
    ) -> Result<BindOAuthResponse, anyhow::Error> {
        // TODO: generate state and build provider authorization URL using oauth crate.
        let _ = req;
        let _ = self.repos.user.find_user_auth_methods(_user_id).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string()
            ))
        })?;
        Ok(BindOAuthResponse {
            redirect: String::new(),
        })
    }
}
