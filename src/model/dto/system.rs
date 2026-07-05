//! API DTO types — system domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::misc::TimePeriod;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub currency_unit: String,
    pub currency_symbol: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyConfig {
    pub access_key: String,
    pub currency_unit: String,
    pub currency_symbol: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNodeMultiplierResponse {
    pub periods: Vec<TimePeriod>,
}


// ─── Invite ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteConfig {
    pub forced_invite: bool,
    pub referral_percentage: i64,
    pub only_first_purchase: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub secret: String,
    pub service_name: String,
    pub service_version: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPolicyConfig {
    pub privacy_policy: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubilcRegisterConfig {
    pub stop_register: bool,
    pub enable_ip_register_limit: bool,
    pub ip_register_limit: i64,
    pub ip_register_limit_duration: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubilcVerifyCodeConfig {
    pub verify_code_interval: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryIPLocationRequest {
    pub ip: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryIPLocationResponse {
    pub country: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub city: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterConfig {
    pub stop_register: bool,
    pub enable_trial: bool,
    pub trial_subscribe: i64,
    pub trial_time: i64,
    pub trial_time_unit: String,
    pub enable_ip_register_limit: bool,
    pub ip_register_limit: i64,
    pub ip_register_limit_duration: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetNodeMultiplierRequest {
    pub periods: Vec<TimePeriod>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub host: String,
    pub site_name: String,
    pub site_desc: String,
    pub site_logo: String,
    pub keywords: String,
    pub custom_html: String,
    pub custom_data: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteCustomDataContacts {
    pub email: String,
    pub telephone: String,
    pub address: String,
}


// ─── Telegram ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub telegram_bot_token: String,
    pub telegram_group_url: String,
    pub telegram_notify: bool,
    pub telegram_web_hook_domain: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TosConfig {
    pub tos_content: String,
}


// ─── Verify / Version / Vless / Vmess / Withdrawal ──────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeifyConfig {
    pub turnstile_site_key: String,
    pub enable_login_verify: bool,
    pub enable_register_verify: bool,
    pub enable_reset_password_verify: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyCodeConfig {
    pub verify_code_expire_time: i64,
    pub verify_code_limit: i64,
    pub verify_code_interval: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyConfig {
    pub turnstile_site_key: String,
    pub turnstile_secret: String,
    pub enable_login_verify: bool,
    pub enable_register_verify: bool,
    pub enable_reset_password_verify: bool,
}
