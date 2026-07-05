//! API DTO types — order domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::payment::{PaymentMethod, StripePayment};
use super::subscribe::Subscribe;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutOrderRequest {
    #[serde(rename = "orderNo")]
    pub order_no: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "returnUrl")]
    pub return_url: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutOrderResponse {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stripe: Option<Box<StripePayment>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseOrderRequest {
    #[serde(rename = "orderNo")]
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: i64,
    #[serde(rename = "type")]
    pub type_: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    pub price: i64,
    pub amount: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coupon_discount: Option<i64>,
    pub commission: i64,
    pub fee_amount: i64,
    pub payment_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderListRequest {
    pub page: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderListResponse {
    pub total: i64,
    pub list: Vec<Order>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub order_no: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub quantity: i64,
    pub price: i64,
    pub amount: i64,
    pub gift_amount: i64,
    pub discount: i64,
    pub coupon: String,
    pub coupon_discount: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commission: Option<i64>,
    pub payment: PaymentMethod,
    pub fee_amount: i64,
    pub trade_no: String,
    pub status: u8,
    pub subscribe_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetail {
    pub id: i64,
    pub user_id: i64,
    pub order_no: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub quantity: i64,
    pub price: i64,
    pub amount: i64,
    pub gift_amount: i64,
    pub discount: i64,
    pub coupon: String,
    pub coupon_discount: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commission: Option<i64>,
    pub payment: PaymentMethod,
    pub method: String,
    pub fee_amount: i64,
    pub trade_no: String,
    pub status: u8,
    pub subscribe_id: i64,
    pub subscribe: Subscribe,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdersStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    pub amount_total: i64,
    pub new_order_amount: i64,
    pub renewal_order_amount: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<OrdersStatistics>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalPurchaseRequest {
    pub auth_type: String,
    pub identifier: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub payment: i64,
    pub subscribe_id: i64,
    pub quantity: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turnstile_token: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalPurchaseResponse {
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreOrderResponse {
    pub price: i64,
    pub amount: i64,
    pub discount: i64,
    pub gift_amount: i64,
    pub coupon: String,
    pub coupon_discount: i64,
    pub fee_amount: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrePurchaseOrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<i64>,
    pub subscribe_id: i64,
    pub quantity: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrePurchaseOrderResponse {
    pub price: i64,
    pub amount: i64,
    pub discount: i64,
    pub coupon: String,
    pub coupon_discount: i64,
    pub fee_amount: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreRenewalOrderResponse {
    #[serde(rename = "orderNo")]
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderRequest {
    pub subscribe_id: i64,
    pub quantity: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseOrderResponse {
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOrderDetailRequest {
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOrderListRequest {
    pub page: i32,
    pub size: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOrderListResponse {
    pub total: i64,
    pub list: Vec<OrderDetail>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPurchaseOrderRequest {
    pub auth_type: String,
    pub identifier: String,
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPurchaseOrderResponse {
    pub order_no: String,
    pub subscribe: Subscribe,
    pub quantity: i64,
    pub price: i64,
    pub amount: i64,
    pub discount: i64,
    pub coupon: String,
    pub coupon_discount: i64,
    pub fee_amount: i64,
    pub payment: PaymentMethod,
    pub status: u8,
    pub created_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}


// ─── Recharge / Register ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RechargeOrderRequest {
    pub amount: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RechargeOrderResponse {
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewalOrderRequest {
    pub user_subscribe_id: i64,
    pub quantity: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewalOrderResponse {
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetTrafficOrderRequest {
    pub user_subscribe_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetTrafficOrderResponse {
    pub order_no: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueStatisticsResponse {
    pub today: OrdersStatistics,
    pub monthly: OrdersStatistics,
    pub all: OrdersStatistics,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub id: i64,
    pub status: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
}
