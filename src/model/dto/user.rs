//! API DTO types — user domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::auth::UserLoginLog;
use super::log::{BalanceLog, CommissionLog};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteUserRequest {
    pub ids: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionWithdrawRequest {
    pub amount: i64,
    pub content: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserAuthMethodRequest {
    pub user_id: i64,
    pub auth_type: String,
    pub auth_identifier: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub telephone: String,
    pub telephone_area_code: String,
    pub password: String,
    pub product_id: i64,
    pub duration: i64,
    pub referral_percentage: u8,
    pub only_first_purchase: bool,
    pub referer_user: String,
    pub refer_code: String,
    pub balance: i64,
    pub commission: i64,
    pub gift_amount: i64,
    pub is_admin: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserAuthMethodRequest {
    pub user_id: i64,
    pub auth_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserDeivceRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeviceListResponse {
    pub list: Vec<UserDevice>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOAuthMethodsResponse {
    pub methods: Vec<UserAuthMethod>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserAuthMethodRequest {
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserAuthMethodResponse {
    pub auth_methods: Vec<UserAuthMethod>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub unscoped: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_subscribe_id: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserListResponse {
    pub total: i64,
    pub list: Vec<User>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserLoginLogsRequest {
    pub page: i32,
    pub size: i32,
    pub user_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserLoginLogsResponse {
    pub list: Vec<UserLoginLog>,
    pub total: i64,
}


// ─── Kick ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KickOfflineRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub platform: String,
    pub platform_url: String,
    pub platform_field_description: std::collections::HashMap<String, String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformResponse {
    pub list: Vec<PlatformInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreUnsubscribeRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreUnsubscribeResponse {
    pub deduction_amount: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserAffiliateCountResponse {
    pub registers: i64,
    pub total_commission: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserAffiliateListRequest {
    pub page: i32,
    pub size: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserAffiliateListResponse {
    pub list: Vec<UserAffiliate>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserBalanceLogListResponse {
    pub list: Vec<BalanceLog>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserCommissionLogListRequest {
    pub page: i32,
    pub size: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryUserCommissionLogListResponse {
    pub list: Vec<CommissionLog>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWithdrawalLogListRequest {
    pub page: i32,
    pub size: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWithdrawalLogListResponse {
    pub list: Vec<WithdrawalLog>,
    pub total: i64,
}


// ─── Unbind / Unsubscribe / Update ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindDeviceRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBindEmailRequest {
    pub email: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBindMobileRequest {
    pub area_code: String,
    pub mobile: String,
    pub code: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserAuthMethodRequest {
    pub user_id: i64,
    pub auth_type: String,
    pub auth_identifier: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserBasiceInfoRequest {
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default)]
    pub balance: i64,
    #[serde(default)]
    pub commission: i64,
    #[serde(default)]
    pub referral_percentage: u8,
    #[serde(default)]
    pub only_first_purchase: bool,
    #[serde(default)]
    pub gift_amount: i64,
    #[serde(default)]
    pub telegram: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refer_code: Option<String>,
    #[serde(default)]
    pub referer_id: i64,
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub is_admin: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserNotifyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_balance_notify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_login_notify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_subscribe_notify: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_trade_notify: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserNotifySettingRequest {
    pub user_id: i64,
    pub enable_balance_notify: bool,
    pub enable_login_notify: bool,
    pub enable_subscribe_notify: bool,
    pub enable_trade_notify: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserPasswordRequest {
    pub password: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRulesRequest {
    pub rules: Vec<String>,
}


// ─── User ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub avatar: String,
    pub balance: i64,
    pub commission: i64,
    pub referral_percentage: u8,
    pub only_first_purchase: bool,
    pub gift_amount: i64,
    pub telegram: i64,
    pub refer_code: String,
    pub referer_id: i64,
    pub enable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    pub enable_balance_notify: bool,
    pub enable_login_notify: bool,
    pub enable_subscribe_notify: bool,
    pub enable_trade_notify: bool,
    pub auth_methods: Vec<UserAuthMethod>,
    pub user_devices: Vec<UserDevice>,
    pub rules: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAffiliate {
    pub avatar: String,
    pub identifier: String,
    pub registered_at: i64,
    pub enable: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthMethod {
    pub auth_type: String,
    pub auth_identifier: String,
    pub verified: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDevice {
    pub id: i64,
    pub ip: String,
    pub identifier: String,
    pub user_agent: String,
    pub online: bool,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    pub register: i64,
    pub new_order_users: i64,
    pub renewal_order_users: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<UserStatistics>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatisticsResponse {
    pub today: UserStatistics,
    pub monthly: UserStatistics,
    pub all: UserStatistics,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTraffic {
    #[serde(rename = "uid")]
    pub sid: i64,
    pub upload: i64,
    pub download: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrafficData {
    pub sid: i64,
    pub upload: i64,
    pub download: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalLog {
    pub id: i64,
    pub user_id: i64,
    pub amount: i64,
    pub content: String,
    pub status: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
