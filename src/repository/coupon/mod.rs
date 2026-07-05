use crate::model::entity::coupon::Coupon;

#[async_trait::async_trait]
pub trait CouponRepo: Send + Sync {
    async fn insert(&self, data: &Coupon) -> Result<Coupon, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Coupon, sqlx::Error>;
    async fn find_one_by_code(&self, code: &str) -> Result<Coupon, sqlx::Error>;
    async fn update(&self, data: &Coupon) -> Result<Coupon, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn update_count(&self, code: &str) -> Result<(), sqlx::Error>;
    async fn query_list_by_page(
        &self,
        page: i64,
        size: i64,
        subscribe: Option<i64>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Coupon>), sqlx::Error>;
    async fn batch_delete(&self, ids: &[i64]) -> Result<u64, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
