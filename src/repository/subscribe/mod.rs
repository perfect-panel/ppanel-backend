use crate::model::entity::subscribe::{Group, Subscribe};

#[derive(Debug, Default)]
pub struct FilterParams {
    pub page: i64,
    pub size: i64,
    pub ids: Vec<i64>,
    pub nodes: Vec<i64>,
    pub tags: Vec<String>,
    pub show: bool,
    pub sell: bool,
    pub language: Option<String>,
    pub default_language: bool,
    pub search: Option<String>,
}

impl FilterParams {
    pub fn normalize(&mut self) {
        if self.page < 1 {
            self.page = 1;
        }
        if self.size < 1 {
            self.size = 10;
        }
    }
}

#[async_trait::async_trait]
pub trait SubscribeRepo: Send + Sync {
    async fn insert(&self, data: &Subscribe) -> Result<Subscribe, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Subscribe, sqlx::Error>;
    async fn update(&self, data: &Subscribe) -> Result<Subscribe, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn create_group(&self, data: &Group) -> Result<Group, sqlx::Error>;
    async fn update_group(&self, data: &Group) -> Result<Group, sqlx::Error>;
    async fn delete_group(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn batch_delete_group(&self, ids: &[i64]) -> Result<u64, sqlx::Error>;
    async fn query_group_list(&self) -> Result<(i64, Vec<Group>), sqlx::Error>;
    async fn update_sort(&self, items: &[Subscribe]) -> Result<(), sqlx::Error>;
    async fn query_reset_cycle_subscribe_ids(
        &self,
        reset_cycle: i64,
    ) -> Result<Vec<i64>, sqlx::Error>;
    async fn query_min_sort_by_ids(&self, ids: &[i64]) -> Result<i64, sqlx::Error>;
    async fn filter_list(
        &self,
        params: &mut FilterParams,
    ) -> Result<(i64, Vec<Subscribe>), sqlx::Error>;
}

pub mod pg;
pub mod mysql;
