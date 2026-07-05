//! API DTO types — announcement domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub id: i64,
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAnnouncementRequest {
    pub title: String,
    pub content: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAnnouncementRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementListRequest {
    pub page: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementListResponse {
    pub total: i64,
    pub list: Vec<Announcement>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnnouncementRequest {
    pub id: i64,
}


// ─── Query Requests ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAnnouncementRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAnnouncementResponse {
    pub total: i64,
    pub announcements: Vec<Announcement>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAnnouncementEnableRequest {
    pub id: i64,
    pub enable: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAnnouncementRequest {
    pub id: i64,
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup: Option<bool>,
}
