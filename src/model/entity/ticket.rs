use serde::{Deserialize, Serialize};

// ─── Status constants ───────────────────────────────────────────────────────

pub const TICKET_STATUS_PENDING: i16 = 1;
pub const TICKET_STATUS_WAITING: i16 = 2;
pub const TICKET_STATUS_PROCESSED: i16 = 3;
pub const TICKET_STATUS_CLOSED: i16 = 4;

// ─── Entity structs ─────────────────────────────────────────────────────────

/// Ticket (`ticket` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ticket {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i64,
    pub status: i16,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Ticket follow-up (`ticket_follow` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Follow {
    pub id: i64,
    pub ticket_id: i64,
    pub from: String,
    #[serde(rename = "type")]
    pub type_: i16,
    pub content: Option<String>,
    pub created_at: i64,
}
