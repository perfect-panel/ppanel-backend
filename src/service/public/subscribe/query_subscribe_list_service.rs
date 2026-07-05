//! List active (show=true, sell=true) subscribe plans.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::entity::subscribe::Subscribe;
use crate::repository::subscribe::FilterParams;
use crate::repository::Repositories;

pub struct QuerySubscribeListService {
    pub repos: Arc<Repositories>,
}

impl QuerySubscribeListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_list(&self, page: i64, size: i64) -> Result<(i64, Vec<Subscribe>), anyhow::Error> {
        let mut params = FilterParams {
            page,
            size,
            show: true,
            sell: true,
            ..Default::default()
        };
        self.repos
            .subscribe
            .filter_list(&mut params)
            .await
            .map_err(|e| anyhow!("query subscribe list: {e}"))
    }
}
