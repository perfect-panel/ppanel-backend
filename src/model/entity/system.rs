use serde::{Deserialize, Serialize};

/// System config key-value entry (`system` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct System {
    pub id: i64,
    pub category: String,
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub desc: String,
    pub created_at: i64,
    pub updated_at: i64,
}
