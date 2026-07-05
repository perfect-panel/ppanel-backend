//! API DTO types — auth domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::system::PubilcRegisterConfig;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppleLoginCallbackRequest {
    pub code: String,
    pub id_token: String,
    pub state: String,
}


// ─── Auth ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub mobile: MobileAuthenticateConfig,
    pub email: EmailAuthticateConfig,
    pub device: DeviceAuthticateConfig,
    pub register: PubilcRegisterConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMethodConfig {
    pub id: i64,
    pub method: String,
    pub config: Value,
    pub enabled: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindOAuthCallbackRequest {
    pub method: String,
    pub callback: Value,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindOAuthRequest {
    pub method: String,
    pub redirect: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindOAuthResponse {
    pub redirect: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindTelegramResponse {
    pub url: String,
    pub expired_at: i64,
}


// ─── Check ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckUserRequest {
    pub email: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckUserResponse {
    pub exist: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckVerificationCodeRequest {
    pub method: String,
    pub account: String,
    pub code: String,
    #[serde(rename = "type")]
    pub type_: u8,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckVerificationCodeRespone {
    pub status: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthticateConfig {
    pub enable: bool,
    pub show_ads: bool,
    pub enable_security: bool,
    pub only_real_device: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceLoginRequest {
    pub identifier: String,
    pub user_agent: String,
    pub cf_token: String,
    #[serde(rename = "X-Original-Forwarded-For")]
    pub ip: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAuthticateConfig {
    pub enable: bool,
    pub enable_verify: bool,
    pub enable_domain_suffix: bool,
    pub domain_suffix_list: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthMethodConfigRequest {
    pub method: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthMethodListResponse {
    pub list: Vec<AuthMethodConfig>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleLoginCallbackRequest {
    pub code: String,
    pub state: String,
}


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
pub struct LoginResponse {
    pub token: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileAuthenticateConfig {
    pub enable: bool,
    pub enable_whitelist: bool,
    pub whitelist: Vec<String>,
}


// ─── OAuth / Online ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAthLoginRequest {
    pub method: String,
    pub redirect: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthLoginGetTokenRequest {
    pub method: String,
    pub callback: Value,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthLoginResponse {
    pub redirect: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub identifier: String,
    pub email: String,
    pub password: String,
    pub code: String,
    #[serde(rename = "X-Original-Forwarded-For")]
    pub ip: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Login-Type")]
    pub login_type: String,
    pub cf_token: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCodeRequest {
    pub email: String,
    #[serde(rename = "type")]
    pub type_: u8,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCodeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub status: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendSmsCodeRequest {
    #[serde(rename = "type")]
    pub type_: u8,
    pub telephone: String,
    pub telephone_area_code: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelephoneCheckUserRequest {
    pub telephone: String,
    pub telephone_area_code: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelephoneCheckUserResponse {
    pub exist: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelephoneLoginRequest {
    pub identifier: String,
    pub telephone: String,
    pub telephone_code: String,
    pub telephone_area_code: String,
    pub password: String,
    #[serde(rename = "X-Original-Forwarded-For")]
    pub ip: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Login-Type")]
    pub login_type: String,
    pub cf_token: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelephoneRegisterRequest {
    pub identifier: String,
    pub telephone: String,
    pub telephone_area_code: String,
    pub password: String,
    pub invite: String,
    pub code: String,
    #[serde(rename = "X-Original-Forwarded-For")]
    pub ip: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Login-Type")]
    pub login_type: String,
    pub cf_token: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelephoneResetPasswordRequest {
    pub identifier: String,
    pub telephone: String,
    pub telephone_area_code: String,
    pub password: String,
    pub code: String,
    #[serde(rename = "X-Original-Forwarded-For")]
    pub ip: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Login-Type")]
    pub login_type: String,
    pub cf_token: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEmailSendRequest {
    pub email: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSmsSendRequest {
    pub area_code: String,
    pub telephone: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnbindOAuthRequest {
    pub method: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAuthMethodConfigRequest {
    pub id: i64,
    pub method: String,
    pub config: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginLog {
    pub id: i64,
    pub user_id: i64,
    pub login_ip: String,
    pub user_agent: String,
    pub success: bool,
    pub timestamp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub identifier: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "X-Original-Forwarded-For")]
    pub ip: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Login-Type")]
    pub login_type: String,
    pub cf_token: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisterRequest {
    pub identifier: String,
    pub email: String,
    pub password: String,
    pub invite: String,
    pub code: String,
    #[serde(rename = "X-Original-Forwarded-For")]
    pub ip: String,
    #[serde(rename = "User-Agent")]
    pub user_agent: String,
    #[serde(rename = "Login-Type")]
    pub login_type: String,
    pub cf_token: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}
