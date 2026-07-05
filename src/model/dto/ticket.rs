//! API DTO types — ticket domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTicketFollowRequest {
    pub ticket_id: i64,
    pub from: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub content: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserTicketFollowRequest {
    pub ticket_id: i64,
    pub from: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub content: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserTicketRequest {
    pub title: String,
    pub description: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follow {
    pub id: i64,
    pub ticket_id: i64,
    pub from: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub content: String,
    pub created_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketListRequest {
    pub page: i64,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketListResponse {
    pub total: i64,
    pub list: Vec<Ticket>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTicketRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTicketDetailRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTicketListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTicketListResponse {
    pub total: i64,
    pub list: Vec<Ticket>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow: Option<Vec<Follow>>,
    pub status: u8,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketWaitRelpyResponse {
    pub count: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTicketStatusRequest {
    pub id: i64,
    pub status: Option<u8>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserTicketStatusRequest {
    pub id: i64,
    pub status: Option<u8>,
}
