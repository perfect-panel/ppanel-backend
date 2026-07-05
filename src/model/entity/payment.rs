use serde::{Deserialize, Serialize};

/// Payment method (`payment` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    pub id: i64,
    pub name: String,
    pub platform: String,
    pub icon: String,
    pub domain: String,
    pub config: String,
    pub description: Option<String>,
    pub fee_mode: i64,
    pub fee_percent: i64,
    pub fee_amount: i64,
    pub sort: i64,
    pub enable: Option<bool>,
    pub token: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// ─── Payment config structs (serialised into `Payment.config`) ──────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StripeConfig {
    pub public_key: String,
    pub secret_key: String,
    pub webhook_secret: String,
    pub payment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AlipayF2FConfig {
    pub app_id: String,
    pub private_key: String,
    pub public_key: String,
    pub invoice_name: String,
    pub sandbox: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EPayConfig {
    pub pid: String,
    pub url: String,
    pub key: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CryptoSaaSConfig {
    pub endpoint: String,
    pub account_id: String,
    pub secret_key: String,
    #[serde(rename = "type")]
    pub type_: String,
}
