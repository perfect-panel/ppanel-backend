use serde::{Deserialize, Serialize};

// 类型别名：表示 Go 的 uint8 (0-255)，但因 PostgreSQL 限制使用 i16
// 应用层应确保值在 0-255 范围内
pub type TinyUint = i16;

/// User (`user` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub password: String,
    pub algo: String,
    pub salt: Option<String>,
    pub avatar: String,  // Go: NOT NULL
    pub balance: i64,
    pub refer_code: String,
    pub referer_id: i64,  // Go: 使用 0 表示无引荐人
    pub commission: i64,
    pub referral_percentage: TinyUint,  // Go uint8: 值范围 0-255
    pub only_first_purchase: bool,  // Go: *bool default:true not null
    pub gift_amount: i64,
    pub enable: bool,  // Go: *bool default:true not null
    pub is_admin: bool,  // Go: *bool default:false not null
    pub enable_balance_notify: bool,  // Go: *bool default:false not null
    pub enable_login_notify: bool,  // Go: *bool default:false not null
    pub enable_subscribe_notify: bool,  // Go: *bool default:false not null
    pub enable_trade_notify: bool,  // Go: *bool default:false not null
    pub rules: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

/// User subscribe (`user_subscribe` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSubscribe {
    pub id: i64,
    pub user_id: i64,
    pub order_id: i64,
    pub subscribe_id: i64,
    pub start_time: i64,
    pub expire_time: i64,
    pub finished_at: Option<i64>,
    pub traffic: i64,
    pub download: i64,
    pub upload: i64,
    pub token: String,
    pub uuid: String,
    pub status: TinyUint,  // Go uint8: 值范围 0-255
    pub note: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// User auth method (`user_auth_methods` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuthMethods {
    pub id: i64,
    pub user_id: i64,
    pub auth_type: String,
    pub auth_identifier: String,
    pub verified: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// User device (`user_device` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Device {
    pub id: i64,
    pub ip: String,
    pub user_id: i64,
    pub user_agent: Option<String>,
    pub identifier: String,
    pub online: bool,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// User device online record (`user_device_online_record` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DeviceOnlineRecord {
    pub id: i64,
    pub user_id: i64,
    pub identifier: String,
    pub online_time: i64,
    pub offline_time: i64,
    pub online_seconds: i64,
    pub duration_days: i64,
    pub created_at: i64,
}

/// User withdrawal (`user_withdrawal` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Withdrawal {
    pub id: i64,
    pub user_id: i64,
    pub amount: i64,
    pub content: Option<String>,
    pub status: TinyUint,  // Go uint8: 值范围 0-255
    pub reason: String,
    pub created_at: i64,
    pub updated_at: i64,
}
