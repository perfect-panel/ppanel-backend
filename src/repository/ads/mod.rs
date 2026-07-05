use crate::model::entity::ads::Ads;

#[async_trait::async_trait]
pub trait AdsRepo: Send + Sync {
    async fn insert(&self, data: &Ads) -> Result<Ads, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Ads, sqlx::Error>;
    async fn update(&self, data: &Ads) -> Result<Ads, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn get_list_by_page(
        &self,
        page: i64,
        size: i64,
        status: Option<i32>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Ads>), sqlx::Error>;
}

pub mod pg;
pub mod mysql;
