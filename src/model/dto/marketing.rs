//! API DTO types — marketing domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSendEmailTask {
    pub id: i64,
    pub subject: String,
    pub content: String,
    pub recipients: String,
    pub scope: i8,
    pub register_start_time: i64,
    pub register_end_time: i64,
    pub additional: String,
    pub scheduled: i64,
    pub interval: u8,
    pub limit: u64,
    pub status: u8,
    pub errors: String,
    pub total: u64,
    pub current: u64,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchSendEmailTaskRequest {
    pub subject: String,
    pub content: String,
    pub scope: i8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register_start_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register_end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuotaTaskRequest {
    pub subscribers: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    pub start_time: i64,
    pub end_time: i64,
    pub reset_traffic: bool,
    pub days: u64,
    pub gift_type: u8,
    pub gift_value: u64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBatchSendEmailTaskListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<i8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBatchSendEmailTaskListResponse {
    pub total: i64,
    pub list: Vec<BatchSendEmailTask>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBatchSendEmailTaskStatusRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBatchSendEmailTaskStatusResponse {
    pub status: u8,
    pub current: i64,
    pub total: i64,
    pub errors: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPreSendEmailCountRequest {
    pub scope: i8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register_start_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub register_end_time: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPreSendEmailCountResponse {
    pub count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuotaTaskListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuotaTaskListResponse {
    pub total: i64,
    pub list: Vec<QuotaTask>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuotaTaskPreCountRequest {
    pub subscribers: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    pub start_time: i64,
    pub end_time: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuotaTaskPreCountResponse {
    pub count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuotaTaskStatusRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryQuotaTaskStatusResponse {
    pub status: u8,
    pub current: i64,
    pub total: i64,
    pub errors: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaTask {
    pub id: i64,
    pub subscribers: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    pub start_time: i64,
    pub end_time: i64,
    pub reset_traffic: bool,
    pub days: u64,
    pub gift_type: u8,
    pub gift_value: u64,
    pub objects: Vec<i64>,
    pub status: u8,
    pub total: i64,
    pub current: i64,
    pub errors: String,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopBatchSendEmailTaskRequest {
    pub id: i64,
}
