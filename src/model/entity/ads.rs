use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ads {
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
    pub created_at: i64,
    pub updated_at: i64,
}
