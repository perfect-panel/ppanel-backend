use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OAuth / email / mobile auth provider configuration stored in the `auth_method` table.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Auth {
    pub id: i64,
    pub method: String,
    pub config: String,
    pub enabled: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}

// ─── Auth config structs (serialised into `Auth.config`) ────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppleAuthConfig {
    pub team_id: String,
    pub key_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GoogleAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GithubAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FacebookAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TelegramAuthConfig {
    pub bot_token: String,
    pub enable_notify: bool,
    pub webhook_domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailAuthConfig {
    pub platform: String,
    pub platform_config: Value,
    pub enable_verify: bool,
    pub enable_notify: bool,
    pub enable_domain_suffix: bool,
    pub domain_suffix_list: String,
    pub verify_email_template: String,
    pub expiration_email_template: String,
    pub maintenance_email_template: String,
    pub traffic_exceed_email_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SmtpConfig {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub pass: String,
    pub from: String,
    pub ssl: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MobileAuthConfig {
    pub platform: String,
    pub platform_config: Value,
    pub enable_whitelist: bool,
    pub whitelist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AlibabaCloudConfig {
    pub access: String,
    pub secret: String,
    pub sign_name: String,
    pub endpoint: String,
    pub template_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SmsbaoConfig {
    pub access: String,
    pub secret: String,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AbosendConfig {
    pub api_domain: String,
    pub access: String,
    pub secret: String,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TwilioConfig {
    pub access: String,
    pub secret: String,
    pub phone_number: String,
    pub template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeviceConfig {
    pub show_ads: bool,
    pub only_real_device: bool,
    pub enable_security: bool,
    pub security_secret: String,
}
