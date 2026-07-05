//! API DTO types — subscribe domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::misc::{StringInt64Slice, TimePeriod};
use super::user::{User, UserDevice};
use super::log::{ResetSubscribeTrafficLog, TrafficLog, UserSubscribeLog};
use super::document::DownloadLink;
use super::server::SortItem;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUserSubcbribe {
    pub id: i64,
    pub name: String,
    pub upload: i64,
    pub traffic: i64,
    pub download: i64,
    pub device_limit: i64,
    pub start_time: String,
    pub expire_time: String,
    pub list: Vec<AppUserSubscbribeNode>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUserSubscbribeNode {
    pub id: i64,
    pub name: String,
    pub uuid: String,
    pub protocol: String,
    pub relay_mode: String,
    pub relay_node: String,
    pub server_addr: String,
    pub speed_limit: i32,
    pub tags: Vec<String>,
    pub traffic: i64,
    pub traffic_ratio: f64,
    pub upload: i64,
    pub config: String,
    pub country: String,
    pub city: String,
    pub latitude: String,
    pub longitude: String,
    pub created_at: i64,
    pub download: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteSubscribeGroupRequest {
    pub ids: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteSubscribeRequest {
    pub ids: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscribeGroupRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscribeRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub unit_price: i64,
    pub unit_time: String,
    pub discount: Vec<SubscribeDiscount>,
    pub replacement: i64,
    pub inventory: i64,
    pub traffic: i64,
    pub speed_limit: i64,
    pub device_limit: i64,
    pub quota: i64,
    pub nodes: StringInt64Slice,
    pub node_tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell: Option<bool>,
    pub deduction_ratio: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deduction: Option<bool>,
    pub reset_cycle: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewal_reset: Option<bool>,
    pub show_original_price: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserSubscribeRequest {
    pub user_id: i64,
    pub expired_at: i64,
    pub traffic: i64,
    pub subscribe_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeGroupRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserSubscribeRequest {
    pub user_subscribe_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeClientResponse {
    pub total: i64,
    pub list: Vec<SubscribeClient>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeDetailsRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeGroupListResponse {
    pub list: Vec<SubscribeGroup>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeListRequest {
    pub page: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeListResponse {
    pub list: Vec<SubscribeItem>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeLogRequest {
    pub page: i32,
    pub size: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeLogResponse {
    pub list: Vec<UserSubscribeLog>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscriptionResponse {
    pub list: Vec<Subscribe>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeByIdRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeDevicesRequest {
    pub page: i32,
    pub size: i32,
    pub user_id: i64,
    pub subscribe_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeDevicesResponse {
    pub list: Vec<UserDevice>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeListRequest {
    pub page: i32,
    pub size: i32,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeListResponse {
    pub list: Vec<UserSubscribe>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeLogsRequest {
    pub page: i32,
    pub size: i32,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeLogsResponse {
    pub list: Vec<UserSubscribeLog>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeResetTrafficLogsRequest {
    pub page: i32,
    pub size: i32,
    pub user_subscribe_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeResetTrafficLogsResponse {
    pub list: Vec<ResetSubscribeTrafficLog>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeTrafficLogsRequest {
    pub page: i32,
    pub size: i32,
    pub user_id: i64,
    pub subscribe_id: i64,
    pub start_time: i64,
    pub end_time: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserSubscribeTrafficLogsResponse {
    pub list: Vec<TrafficLog>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySubscribeGroupListResponse {
    pub list: Vec<SubscribeGroup>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySubscribeListRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySubscribeListResponse {
    pub list: Vec<Subscribe>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserSubscribeListResponse {
    pub list: Vec<UserSubscribe>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserSubscribeNodeListResponse {
    pub list: Vec<UserSubscribeInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetAllSubscribeTokenResponse {
    pub success: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetUserSubscribeTokenRequest {
    pub user_subscribe_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetUserSubscribeTrafficRequest {
    pub user_subscribe_id: i64,
}


// ─── Subscribe ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscribe {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub unit_price: i64,
    pub unit_time: String,
    pub discount: Vec<SubscribeDiscount>,
    pub replacement: i64,
    pub inventory: i64,
    pub traffic: i64,
    pub speed_limit: i64,
    pub device_limit: i64,
    pub quota: i64,
    pub nodes: StringInt64Slice,
    pub node_tags: Vec<String>,
    pub show: bool,
    pub sell: bool,
    pub sort: i64,
    pub deduction_ratio: i64,
    pub allow_deduction: bool,
    pub reset_cycle: i64,
    pub renewal_reset: bool,
    pub show_original_price: bool,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeApplication {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    pub user_agent: String,
    pub is_default: bool,
    pub template: String,
    pub output_format: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_link: Option<DownloadLink>,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeClient {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    pub is_default: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_link: Option<DownloadLink>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeConfig {
    pub single_model: bool,
    pub subscribe_path: String,
    pub subscribe_domain: String,
    pub pan_domain: bool,
    pub user_agent_limit: bool,
    pub user_agent_list: String,
    pub show_tutorial: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeDiscount {
    pub quantity: i64,
    pub discount: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeGroup {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeItem {
    #[serde(flatten)]
    pub subscribe: Subscribe,
    pub sold: i64,
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
pub struct SubscribeSortRequest {
    pub sort: Vec<SortItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeType {
    pub subscribe_types: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleUserSubscribeStatusRequest {
    pub user_subscribe_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscribeGroupRequest {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscribeRequest {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub unit_price: i64,
    pub unit_time: String,
    pub discount: Vec<SubscribeDiscount>,
    pub replacement: i64,
    pub inventory: i64,
    pub traffic: i64,
    pub speed_limit: i64,
    pub device_limit: i64,
    pub quota: i64,
    pub nodes: StringInt64Slice,
    pub node_tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell: Option<bool>,
    pub sort: i64,
    pub deduction_ratio: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_deduction: Option<bool>,
    pub reset_cycle: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewal_reset: Option<bool>,
    pub show_original_price: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserSubscribeNoteRequest {
    pub user_subscribe_id: i64,
    pub note: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserSubscribeRequest {
    pub user_subscribe_id: i64,
    pub subscribe_id: i64,
    pub traffic: i64,
    pub expired_at: i64,
    pub upload: i64,
    pub download: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscribe {
    pub id: i64,
    pub user_id: i64,
    pub order_id: i64,
    pub subscribe_id: i64,
    pub subscribe: Subscribe,
    pub start_time: i64,
    pub expire_time: i64,
    pub finished_at: i64,
    pub reset_time: i64,
    pub traffic: i64,
    pub download: i64,
    pub upload: i64,
    pub token: String,
    pub status: u8,
    pub short: String,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscribeDetail {
    pub id: i64,
    pub user_id: i64,
    pub user: User,
    pub order_id: i64,
    pub subscribe_id: i64,
    pub subscribe: Subscribe,
    pub start_time: i64,
    pub expire_time: i64,
    pub reset_time: i64,
    pub traffic: i64,
    pub download: i64,
    pub upload: i64,
    pub token: String,
    pub status: u8,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscribeInfo {
    pub id: i64,
    pub user_id: i64,
    pub order_id: i64,
    pub subscribe_id: i64,
    pub start_time: i64,
    pub expire_time: i64,
    pub finished_at: i64,
    pub reset_time: i64,
    pub traffic: i64,
    pub download: i64,
    pub upload: i64,
    pub token: String,
    pub status: u8,
    pub created_at: i64,
    pub updated_at: i64,
    pub is_try_out: bool,
    pub nodes: Vec<UserSubscribeNodeInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSubscribeNodeInfo {
    pub id: i64,
    pub name: String,
    pub uuid: String,
    pub protocol: String,
    pub port: u16,
    pub address: String,
    pub tags: Vec<String>,
    pub country: String,
    pub city: String,
    pub created_at: i64,
}
