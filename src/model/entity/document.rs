use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Document {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub tags: String,
    pub show: Option<bool>,
    pub created_at: i64,
    pub updated_at: i64,
}
