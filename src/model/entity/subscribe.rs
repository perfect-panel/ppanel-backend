use serde::{Deserialize, Serialize};

/// Subscribe plan (`subscribe` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Subscribe {
    pub id: i64,
    pub name: String,
    pub language: String,
    pub description: Option<String>,
    pub unit_price: i64,
    pub unit_time: String,
    pub discount: String,  // 修复: Go NOT NULL → Rust String（而非 Option）
    pub replacement: i64,
    pub inventory: i64,
    pub traffic: i64,
    pub speed_limit: i64,
    pub device_limit: i64,
    pub quota: i64,
    pub nodes: String,  // 修复: Go NOT NULL → Rust String（而非 Option）
    pub node_tags: String,  // 修复: Go NOT NULL → Rust String（而非 Option）
    pub show: bool,  // 修复: Go *bool default:0 not null → 总是有值
    pub sell: bool,  // 修复: Go *bool default:0 not null → 总是有值
    pub sort: i64,
    pub deduction_ratio: i64,
    pub allow_deduction: bool,  // 修复: Go *bool default:1 not null → 总是有值
    pub reset_cycle: i64,
    pub renewal_reset: bool,  // 修复: Go *bool default:0 not null → 总是有值
    pub show_original_price: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Discount tier (serialised within `Subscribe.discount`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Discount {
    pub months: i64,
    pub discount: i64,
}

/// Subscribe group (`subscribe_group` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
