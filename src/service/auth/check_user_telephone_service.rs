use std::sync::Arc;

use anyhow::anyhow;

use crate::config::Config;
use crate::model::dto::auth::{TelephoneCheckUserRequest, TelephoneCheckUserResponse};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct CheckUserTelephoneService {
    repos: Arc<Repositories>,
    _config: Arc<Config>,
}

impl CheckUserTelephoneService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self {
            repos,
            _config: config,
        }
    }

    pub async fn check(
        &self,
        req: TelephoneCheckUserRequest,
    ) -> Result<TelephoneCheckUserResponse, anyhow::Error> {
        let phone = format!("+{}{}", req.telephone_area_code, req.telephone);

        let auth_method = self
            .repos
            .user
            .find_auth_method_by_open_id("mobile", &phone)
            .await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        Ok(TelephoneCheckUserResponse {
            exist: auth_method.is_some(),
        })
    }
}
