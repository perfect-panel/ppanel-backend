use crate::model::entity::payment::Payment;

#[derive(Debug, Default)]
pub struct PaymentFilter {
    pub enable: Option<bool>,
    pub platform: Option<String>,
    pub search: Option<String>,
}

#[async_trait::async_trait]
pub trait PaymentRepo: Send + Sync {
    async fn insert(&self, data: &Payment) -> Result<Payment, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Payment, sqlx::Error>;
    async fn find_one_by_token(&self, token: &str) -> Result<Payment, sqlx::Error>;
    async fn update(&self, data: &Payment) -> Result<Payment, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn find_all(&self) -> Result<Vec<Payment>, sqlx::Error>;
    async fn find_available_methods(&self) -> Result<Vec<Payment>, sqlx::Error>;
    async fn find_list_by_page(
        &self,
        page: i64,
        size: i64,
        filter: Option<&PaymentFilter>,
    ) -> Result<(i64, Vec<Payment>), sqlx::Error>;
}

pub mod pg;
pub mod mysql;
