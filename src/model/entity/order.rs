use serde::{Deserialize, Serialize};

// 类型别名：表示 Go 的 uint8 (0-255)，但因 PostgreSQL 限制使用 i16
pub type TinyUint = i16;

/// Order (`order` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub user_id: i64,
    pub order_no: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub type_: TinyUint,  // Go uint8: 值范围 0-255
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
    pub status: TinyUint,  // Go uint8: 值范围 0-255
    pub subscribe_id: i64,
    pub subscribe_token: Option<String>,
    pub is_new: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Aggregated order totals (used in queries, not a table).
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrdersTotal {
    pub amount_total: i64,
    pub new_order_amount: i64,
    pub renewal_order_amount: i64,
}
