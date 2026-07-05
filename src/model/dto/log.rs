//! API DTO types — log domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::auth::UserLoginLog;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceLog {
    #[serde(rename = "type")]
    pub type_: u16,
    pub user_id: i64,
    pub amount: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
    pub balance: i64,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionLog {
    #[serde(rename = "type")]
    pub type_: u16,
    pub user_id: i64,
    pub amount: i64,
    pub order_no: String,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterLogParams {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterBalanceLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterBalanceLogResponse {
    pub total: i64,
    pub list: Vec<BalanceLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCommissionLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCommissionLogResponse {
    pub total: i64,
    pub list: Vec<CommissionLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterEmailLogResponse {
    pub total: i64,
    pub list: Vec<MessageLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterGiftLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterGiftLogResponse {
    pub total: i64,
    pub list: Vec<GiftLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterLoginLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterLoginLogResponse {
    pub total: i64,
    pub list: Vec<LoginLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterMobileLogResponse {
    pub total: i64,
    pub list: Vec<MessageLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRegisterLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRegisterLogResponse {
    pub total: i64,
    pub list: Vec<RegisterLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterResetSubscribeLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_subscribe_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterResetSubscribeLogResponse {
    pub total: i64,
    pub list: Vec<ResetSubscribeLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterServerTrafficLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterServerTrafficLogResponse {
    pub total: i64,
    pub list: Vec<ServerTrafficLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSubscribeLogRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_subscribe_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSubscribeLogResponse {
    pub total: i64,
    pub list: Vec<SubscribeLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSubscribeTrafficRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_subscribe_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSubscribeTrafficResponse {
    pub total: i64,
    pub list: Vec<UserSubscribeTrafficLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterTrafficLogDetailsRequest {
    #[serde(flatten)]
    pub params: FilterLogParams,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterTrafficLogDetailsResponse {
    pub total: i64,
    pub list: Vec<TrafficLogDetails>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLoginLogRequest {
    pub page: i32,
    pub size: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLoginLogResponse {
    pub list: Vec<UserLoginLog>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessageLogListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(rename = "type")]
    pub type_: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessageLogListResponse {
    pub total: i64,
    pub list: Vec<MessageLog>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftLog {
    #[serde(rename = "type")]
    pub type_: u16,
    pub user_id: i64,
    pub order_no: String,
    pub subscribe_id: i64,
    pub amount: i64,
    pub balance: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    pub timestamp: i64,
}


// ─── Log ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLog {
    pub user_id: i64,
    pub method: String,
    pub login_ip: String,
    pub user_agent: String,
    pub success: bool,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogResponse {
    pub list: Value,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_clear: Option<bool>,
    pub clear_days: i64,
}


// ─── Message ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageLog {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: u8,
    pub platform: String,
    pub to: String,
    pub subject: String,
    pub content: Value,
    pub status: u8,
    pub created_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterLog {
    pub user_id: i64,
    pub auth_method: String,
    pub identifier: String,
    pub register_ip: String,
    pub user_agent: String,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetSubscribeLog {
    #[serde(rename = "type")]
    pub type_: u16,
    pub user_id: i64,
    pub user_subscribe_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeLog {
    pub user_id: i64,
    pub token: String,
    pub user_agent: String,
    pub client_ip: String,
    pub user_subscribe_id: i64,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetSubscribeTrafficLog {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: u16,
    pub user_subscribe_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTrafficLog {
    pub server_id: i64,
    pub upload: i64,
    pub download: i64,
    pub total: i64,
    pub date: String,
    pub details: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficLog {
    pub id: i64,
    pub server_id: i64,
    pub user_id: i64,
    pub subscribe_id: i64,
    pub download: i64,
    pub upload: i64,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficLogDetails {
    pub id: i64,
    pub server_id: i64,
    pub user_id: i64,
    pub subscribe_id: i64,
    pub download: i64,
    pub upload: i64,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscribeLog {
    pub id: i64,
    pub user_id: i64,
    pub user_subscribe_id: i64,
    pub token: String,
    pub ip: String,
    pub user_agent: String,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscribeTrafficLog {
    pub subscribe_id: i64,
    pub user_id: i64,
    pub upload: i64,
    pub download: i64,
    pub total: i64,
    pub date: String,
    pub details: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalLog {
    pub id: i64,
    pub user_id: i64,
    pub amount: i64,
    pub content: Option<String>,
    pub status: u8,
    pub reason: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWithdrawalLogListRequest {
    pub page: i32,
    pub size: i32,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWithdrawalLogListResponse {
    pub total: i64,
    pub list: Vec<WithdrawalLog>,
}
