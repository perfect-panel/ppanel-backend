use crate::model::entity::order::{Order, OrdersTotal};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderDetails {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub user_id: i64,
    pub order_no: String,
    #[sqlx(rename = "type")]
    pub type_: i16,
    pub quantity: i64,
    pub price: i64,
    pub amount: i64,
    pub gift_amount: i64,
    pub discount: i64,
    pub coupon: Option<String>,
    pub coupon_discount: i64,
    pub commission: i64,
    pub payment_id: i64,
    pub method: String,
    pub fee_amount: i64,
    pub trade_no: Option<String>,
    pub status: i16,
    pub subscribe_id: i64,
    pub subscribe_token: Option<String>,
    pub is_new: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub subscribe_name: Option<String>,
    pub payment_name: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrdersTotalWithDate {
    pub date: String,
    pub amount_total: i64,
    pub new_order_amount: i64,
    pub renewal_order_amount: i64,
}

#[async_trait::async_trait]
pub trait OrderRepo: Send + Sync {
    async fn insert(&self, data: &Order) -> Result<Order, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Order, sqlx::Error>;
    async fn find_one_by_order_no(&self, order_no: &str) -> Result<Order, sqlx::Error>;
    async fn find_one_details(&self, id: i64) -> Result<OrderDetails, sqlx::Error>;
    async fn find_one_details_by_order_no(&self, order_no: &str) -> Result<OrderDetails, sqlx::Error>;
    async fn update(&self, data: &Order) -> Result<Order, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn update_order_status(&self, order_no: &str, status: i16) -> Result<u64, sqlx::Error>;
    async fn count_user_coupon_usage(&self, user_id: i64, coupon: &str) -> Result<i64, sqlx::Error>;
    async fn query_list_by_page(
        &self,
        page: i64,
        size: i64,
        status: i16,
        user_id: i64,
        subscribe_id: i64,
        search: Option<&str>,
    ) -> Result<(i64, Vec<OrderDetails>), sqlx::Error>;
    async fn is_user_eligible_for_new_order(&self, user_id: i64) -> Result<bool, sqlx::Error>;
    async fn query_monthly_orders(&self, now: i64) -> Result<OrdersTotal, sqlx::Error>;
    async fn query_date_orders(&self, date: i64) -> Result<OrdersTotal, sqlx::Error>;
    async fn query_total_orders(&self) -> Result<OrdersTotal, sqlx::Error>;
    async fn query_daily_orders_list(&self, now: i64) -> Result<Vec<OrdersTotalWithDate>, sqlx::Error>;
    async fn query_monthly_orders_list(&self, now: i64) -> Result<Vec<OrdersTotalWithDate>, sqlx::Error>;
    async fn query_monthly_user_counts(&self, now: i64) -> Result<(i64, i64), sqlx::Error>;
    async fn query_date_user_counts(&self, date: i64) -> Result<(i64, i64), sqlx::Error>;
    async fn query_total_user_counts(&self) -> Result<(i64, i64), sqlx::Error>;
}

pub mod pg;
pub mod mysql;
