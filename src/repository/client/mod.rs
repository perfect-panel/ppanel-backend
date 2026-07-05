use crate::model::entity::client::SubscribeApplication;

#[async_trait::async_trait]
pub trait ClientRepo: Send + Sync {
    async fn insert(&self, data: &SubscribeApplication) -> Result<SubscribeApplication, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<SubscribeApplication, sqlx::Error>;
    async fn update(&self, data: &SubscribeApplication) -> Result<SubscribeApplication, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn list(&self) -> Result<Vec<SubscribeApplication>, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
