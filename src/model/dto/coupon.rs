//! API DTO types — coupon domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteCouponRequest {
    pub ids: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub count: i64,
    #[serde(rename = "type")]
    pub type_: u8,
    pub discount: i64,
    pub start_time: i64,
    pub expire_time: i64,
    pub user_limit: i64,
    pub subscribe: Vec<i64>,
    pub used_count: i64,
    pub enable: bool,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCouponRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "type")]
    pub type_: u8,
    pub discount: i64,
    pub start_time: i64,
    pub expire_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCouponRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCouponListRequest {
    pub page: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCouponListResponse {
    pub total: i64,
    pub list: Vec<Coupon>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCouponRequest {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "type")]
    pub type_: u8,
    pub discount: i64,
    pub start_time: i64,
    pub expire_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}
