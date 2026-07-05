use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::user::QueryUserAffiliateCountResponse;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryUserAffiliateService {
    repos: Arc<Repositories>,
}

impl QueryUserAffiliateService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_user_affiliate(
        &self,
        user_id: i64,
    ) -> Result<QueryUserAffiliateCountResponse, anyhow::Error> {
        let registers = self
            .repos
            .user
            .count_affiliates(user_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let user = self.repos.user.find_one_user(user_id).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string()
            ))
        })?;

        Ok(QueryUserAffiliateCountResponse {
            registers,
            total_commission: user.commission,
        })
    }
}
