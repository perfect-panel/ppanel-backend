//! API DTO types — payment domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlipayNotifyResponse {
    pub return_code: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentMethodRequest {
    pub name: String,
    pub platform: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    pub config: Value,
    pub fee_mode: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<i64>,
    pub enable: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePaymentMethodRequest {
    pub id: i64,
}


// ─── E-Pay / Email Auth / Email ─────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EPayNotifyRequest {
    pub pid: i64,
    pub trade_no: String,
    pub out_trade_no: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub money: String,
    pub trade_status: String,
    pub param: String,
    pub sign: String,
    pub sign_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAvailablePaymentMethodsResponse {
    pub list: Vec<PaymentMethod>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentMethodListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentMethodListResponse {
    pub total: i64,
    pub list: Vec<PaymentMethodDetail>,
}


// ─── Payment ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConfig {
    pub id: i64,
    pub name: String,
    pub platform: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    pub config: Value,
    pub fee_mode: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<i64>,
    pub enable: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: i64,
    pub name: String,
    pub platform: String,
    pub description: String,
    pub icon: String,
    pub fee_mode: u32,
    pub fee_percent: i64,
    pub fee_amount: i64,
    pub sort: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetail {
    pub id: i64,
    pub name: String,
    pub platform: String,
    pub description: String,
    pub icon: String,
    pub domain: String,
    pub config: Value,
    pub fee_mode: u32,
    pub fee_percent: i64,
    pub fee_amount: i64,
    pub sort: i64,
    pub enable: bool,
    pub notify_url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StripePayment {
    pub method: String,
    pub client_secret: String,
    pub publishable_key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentMethodRequest {
    pub id: i64,
    pub name: String,
    pub platform: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    pub config: Value,
    pub fee_mode: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_percent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<i64>,
    pub enable: Option<bool>,
}
