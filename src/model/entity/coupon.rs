use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Coupon {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub count: i64,
    #[serde(rename = "type")]
    pub type_: i16,
    pub discount: i64,
    pub start_time: i64,
    pub expire_time: i64,
    pub user_limit: i64,
    pub subscribe: String,
    pub used_count: i64,
    pub enable: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}
