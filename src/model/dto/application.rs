//! API DTO types — application domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::document::DownloadLink;
use super::subscribe::SubscribeApplication;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: i64,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub subscribe_type: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationPlatform {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios: Option<Vec<ApplicationVersion>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub macos: Option<Vec<ApplicationVersion>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<Vec<ApplicationVersion>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android: Option<Vec<ApplicationVersion>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<Vec<ApplicationVersion>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harmony: Option<Vec<ApplicationVersion>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationResponse {
    pub applications: Vec<ApplicationResponseInfo>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationResponseInfo {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub subscribe_type: String,
    pub platform: ApplicationPlatform,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationVersion {
    pub id: i64,
    pub url: String,
    pub version: String,
    pub description: String,
    pub is_default: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscribeApplicationRequest {
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
    pub download_link: DownloadLink,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscribeApplicationRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeApplicationListRequest {
    pub page: i32,
    pub size: i32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSubscribeApplicationListResponse {
    pub total: i64,
    pub list: Vec<SubscribeApplication>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewSubscribeTemplateRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewSubscribeTemplateResponse {
    pub template: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSubscribeApplicationRequest {
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
}
