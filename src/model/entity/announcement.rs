use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Announcement {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub show: Option<bool>,
    pub pinned: Option<bool>,
    pub popup: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}
