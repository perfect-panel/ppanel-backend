use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::user::{QueryUserAffiliateListRequest, QueryUserAffiliateListResponse, UserAffiliate};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryUserAffiliateListService {
    repos: Arc<Repositories>,
}

impl QueryUserAffiliateListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_user_affiliate_list(
        &self,
        user_id: i64,
        req: QueryUserAffiliateListRequest,
    ) -> Result<QueryUserAffiliateListResponse, anyhow::Error> {
        let page = req.page.max(1) as i64;
        let size = req.size.max(10) as i64;

        let (total, users) = self
            .repos
            .user
            .query_affiliate_list(user_id, page, size)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let list = users
            .into_iter()
            .map(|u| UserAffiliate {
                avatar: u.avatar,
                identifier: u.refer_code,
                registered_at: u.created_at,
                enable: u.enable,
            })
            .collect();

        Ok(QueryUserAffiliateListResponse { list, total })
    }
}
