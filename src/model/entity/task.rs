use serde::{Deserialize, Serialize};

// ─── Task type constants ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskType(pub i8);

impl TaskType {
    pub const EMAIL: Self = Self(0);
    pub const QUOTA: Self = Self(1);
}

// ─── Scope type constants ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeType(pub i8);

impl ScopeType {
    pub const ALL: Self = Self(1);
    pub const ACTIVE: Self = Self(2);
    pub const EXPIRED: Self = Self(3);
    pub const NONE: Self = Self(4);
    pub const SKIP: Self = Self(5);
}

// ─── Entity structs ─────────────────────────────────────────────────────────

/// Background task (`task` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: i16,
    pub scope: Option<String>,
    pub content: Option<String>,
    pub status: i16,
    pub errors: Option<String>,
    pub total: i64,
    pub current: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

// ─── Task content / scope structs (serialised within `Task`) ────────────────

/// Email batch task scope.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailScope {
    #[serde(rename = "type")]
    pub type_: i16,
    #[serde(default)]
    pub register_start_time: i64,
    #[serde(default)]
    pub register_end_time: i64,
    #[serde(default)]
    pub recipients: Vec<String>,
    #[serde(default)]
    pub additional: Vec<String>,
    #[serde(default)]
    pub scheduled: i64,
    #[serde(default)]
    pub interval: i16,
    #[serde(default)]
    pub limit: i64,
}

/// Email batch task content.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailContent {
    pub subject: String,
    pub content: String,
}

/// Quota task scope.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuotaScope {
    #[serde(default)]
    pub subscribers: Vec<i64>,
    pub is_active: Option<bool>,
    #[serde(default)]
    pub start_time: i64,
    #[serde(default)]
    pub end_time: i64,
    #[serde(default)]
    pub recipients: Vec<i64>,
}

/// Quota task content.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuotaContent {
    pub reset_traffic: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gift_type: Option<i16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gift_value: Option<i64>,
}
