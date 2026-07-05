use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::user::GetOAuthMethodsResponse;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetOAuthMethodsService {
    repos: Arc<Repositories>,
}

impl GetOAuthMethodsService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get_o_auth_methods(
        &self,
        user_id: i64,
    ) -> Result<GetOAuthMethodsResponse, anyhow::Error> {
        let methods = self
            .repos
            .user
            .find_user_auth_methods(user_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let list = methods
            .into_iter()
            .map(|m| crate::model::dto::user::UserAuthMethod {
                auth_type: m.auth_type,
                auth_identifier: m.auth_identifier,
                verified: m.verified,
            })
            .collect();

        Ok(GetOAuthMethodsResponse { methods: list })
    }
}
