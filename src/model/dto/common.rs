//! API DTO types — common domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::auth::AuthConfig;
use super::subscribe::SubscribeConfig;
use super::system::{Currency, InviteConfig, PubilcVerifyCodeConfig, SiteConfig, VeifyConfig};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDetailRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGlobalConfigResponse {
    pub site: SiteConfig,
    pub verify: VeifyConfig,
    pub auth: AuthConfig,
    pub invite: InviteConfig,
    pub currency: Currency,
    pub subscribe: SubscribeConfig,
    pub verify_code: PubilcVerifyCodeConfig,
    pub oauth_methods: Vec<String>,
    pub web_ad: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatResponse {
    pub user: i64,
    pub node: i64,
    pub country: i64,
    pub protocol: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTosResponse {
    pub tos_content: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatResponse {
    pub status: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}
