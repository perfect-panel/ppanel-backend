//! API DTO types — document domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteDocumentRequest {
    pub ids: Vec<i64>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentRequest {
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocumentRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub show: bool,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub harmony: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentDetailRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentListRequest {
    pub page: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentListResponse {
    pub total: i64,
    pub list: Vec<Document>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDocumentDetailRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDocumentListResponse {
    pub total: i64,
    pub list: Vec<Document>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentRequest {
    pub id: i64,
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
}
