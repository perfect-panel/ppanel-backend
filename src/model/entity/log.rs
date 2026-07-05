use serde::{Deserialize, Serialize};

// ─── Log type constants ─────────────────────────────────────────────────────

/// Log Types:
///   1X Message Logs
///   2X Subscription Logs
///   3X User Logs
///   4X Traffic Ranking Logs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogType(pub i16);

impl LogType {
    pub const EMAIL_MESSAGE: Self = Self(10);
    pub const MOBILE_MESSAGE: Self = Self(11);
    pub const SUBSCRIBE: Self = Self(20);
    pub const SUBSCRIBE_TRAFFIC: Self = Self(21);
    pub const SERVER_TRAFFIC: Self = Self(22);
    pub const RESET_SUBSCRIBE: Self = Self(23);
    pub const LOGIN: Self = Self(30);
    pub const REGISTER: Self = Self(31);
    pub const BALANCE: Self = Self(32);
    pub const COMMISSION: Self = Self(33);
    pub const GIFT: Self = Self(34);
    pub const USER_TRAFFIC_RANK: Self = Self(40);
    pub const SERVER_TRAFFIC_RANK: Self = Self(41);
    pub const TRAFFIC_STAT: Self = Self(42);
}

// ─── Sub-type constants ─────────────────────────────────────────────────────

pub const RESET_SUBSCRIBE_TYPE_AUTO: i32 = 231;
pub const RESET_SUBSCRIBE_TYPE_ADVANCE: i32 = 232;
pub const RESET_SUBSCRIBE_TYPE_PAID: i32 = 233;
pub const RESET_SUBSCRIBE_TYPE_QUOTA: i32 = 234;
pub const BALANCE_TYPE_RECHARGE: i32 = 321;
pub const BALANCE_TYPE_WITHDRAW: i32 = 322;
pub const BALANCE_TYPE_PAYMENT: i32 = 323;
pub const BALANCE_TYPE_REFUND: i32 = 324;
pub const BALANCE_TYPE_REWARD: i32 = 325;
pub const BALANCE_TYPE_ADJUST: i32 = 326;
pub const COMMISSION_TYPE_PURCHASE: i32 = 331;
pub const COMMISSION_TYPE_RENEWAL: i32 = 332;
pub const COMMISSION_TYPE_REFUND: i32 = 333;
pub const COMMISSION_TYPE_WITHDRAW: i32 = 334;
pub const COMMISSION_TYPE_ADJUST: i32 = 335;
pub const COMMISSION_TYPE_CONVERT_BALANCE: i32 = 336;
pub const GIFT_TYPE_INCREASE: i32 = 341;
pub const GIFT_TYPE_REDUCE: i32 = 342;

// ─── Entity structs ─────────────────────────────────────────────────────────

/// System log entry (`system_logs` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SystemLog {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: i16,
    pub date: Option<String>,
    pub object_id: i64,
    pub content: String,
    pub created_at: i64,
}

/// Message log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub content: serde_json::Value,
    pub platform: String,
    pub template: String,
    pub status: i16,
}

/// Traffic log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Traffic {
    pub download: i64,
    pub upload: i64,
}

/// Login log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Login {
    pub method: String,
    pub login_ip: String,
    pub user_agent: String,
    pub success: bool,
    pub timestamp: i64,
}

/// Registration log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Register {
    pub auth_method: String,
    pub identifier: String,
    pub register_ip: String,
    pub user_agent: String,
    pub timestamp: i64,
}

/// Subscription log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SubscribeLog {
    pub token: String,
    pub user_agent: String,
    pub client_ip: String,
    pub user_subscribe_id: i64,
}

/// Reset subscribe log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ResetSubscribe {
    #[serde(rename = "type")]
    pub type_: i32,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
    pub timestamp: i64,
}

/// Balance log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Balance {
    #[serde(rename = "type")]
    pub type_: i32,
    pub amount: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_no: Option<String>,
    pub balance: i64,
    pub timestamp: i64,
}

/// Commission log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Commission {
    #[serde(rename = "type")]
    pub type_: i32,
    pub amount: i64,
    pub order_no: String,
    pub timestamp: i64,
}

/// Gift log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Gift {
    #[serde(rename = "type")]
    pub type_: i32,
    pub order_no: String,
    pub subscribe_id: i64,
    pub amount: i64,
    pub balance: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    pub timestamp: i64,
}

/// User traffic log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserTraffic {
    pub subscribe_id: i64,
    pub user_id: i64,
    pub upload: i64,
    pub download: i64,
    pub total: i64,
}

/// User traffic rank entry.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserTrafficRank {
    pub rank: std::collections::HashMap<u8, UserTraffic>,
}

/// Server traffic log content (serialised within `SystemLog.content`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerTraffic {
    pub server_id: i64,
    pub upload: i64,
    pub download: i64,
    pub total: i64,
}

/// Server traffic rank entry.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerTrafficRank {
    pub rank: std::collections::HashMap<u8, ServerTraffic>,
}

/// Daily traffic statistics.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrafficStat {
    pub upload: i64,
    pub download: i64,
    pub total: i64,
}
