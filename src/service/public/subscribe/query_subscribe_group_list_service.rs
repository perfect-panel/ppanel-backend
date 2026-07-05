//! List subscribe groups.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::entity::subscribe::Group;
use crate::repository::Repositories;

pub struct QuerySubscribeGroupListService {
    pub repos: Arc<Repositories>,
}

impl QuerySubscribeGroupListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_list(&self) -> Result<(i64, Vec<Group>), anyhow::Error> {
        self.repos
            .subscribe
            .query_group_list()
            .await
            .map_err(|e| anyhow!("query subscribe group list: {e}"))
    }
}
