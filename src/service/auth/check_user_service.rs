use std::sync::Arc;

use anyhow::anyhow;

use crate::config::Config;
use crate::model::dto::auth::{CheckUserRequest, CheckUserResponse};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct CheckUserService {
    repos: Arc<Repositories>,
    _config: Arc<Config>,
}

impl CheckUserService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self {
            repos,
            _config: config,
        }
    }

    pub async fn check(&self, req: CheckUserRequest) -> Result<CheckUserResponse, anyhow::Error> {
        let auth_method = self
            .repos
            .user
            .find_auth_method_by_open_id("email", &req.email)
            .await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        Ok(CheckUserResponse {
            exist: auth_method.is_some(),
        })
    }
}
