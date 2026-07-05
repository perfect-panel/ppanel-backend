use crate::model::entity::auth::Auth;

#[async_trait::async_trait]
pub trait AuthRepo: Send + Sync {
    async fn insert(&self, data: &Auth) -> Result<Auth, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Auth, sqlx::Error>;
    async fn update(&self, data: &Auth) -> Result<Auth, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn get_list(&self) -> Result<Vec<Auth>, sqlx::Error>;
    async fn find_one_by_method(&self, method: &str) -> Result<Auth, sqlx::Error>;
    async fn find_all_enabled(&self) -> Result<Vec<Auth>, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
