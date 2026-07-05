//! API DTO types — ads domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


// ─── Ads ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ads {
    pub id: i32,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
    pub description: String,
    pub target_url: String,
    pub start_time: i64,
    pub end_time: i64,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}


// ─── Create Requests ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAdsRequest {
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
    pub description: String,
    pub target_url: String,
    pub start_time: i64,
    pub end_time: i64,
    pub status: i32,
}


// ─── Delete Requests ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAdsRequest {
    pub id: i64,
}


// ─── Get Requests ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAdsDetailRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAdsListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAdsListResponse {
    pub total: i64,
    pub list: Vec<Ads>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAdsRequest {
    pub device: String,
    pub position: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAdsResponse {
    pub list: Vec<Ads>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAdsRequest {
    pub id: i64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub content: String,
    pub description: String,
    pub target_url: String,
    pub start_time: i64,
    pub end_time: i64,
    pub status: i32,
}
