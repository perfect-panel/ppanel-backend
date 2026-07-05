//! Redis / cache key constants, ported from `server/internal/config/cacheKey.go`.
//!
//! TODO: these constants are defined ahead of the cache layer. The root
//! `Cargo.toml` does not yet pull a Redis client (e.g. `redis` / `deadpool-redis`)
//! and `config::RedisConfig` is a dead leaf until the cache service is wired
//! up. When introducing the cache layer, add the dependency and a `cache`
//! module that consumes these keys.

pub const CURRENCY_CONFIG_KEY: &str = "system:currency_config";
pub const SMS_CONFIG_KEY: &str = "system:sms_config";
pub const SITE_CONFIG_KEY: &str = "system:site_config";
pub const SUBSCRIBE_CONFIG_KEY: &str = "system:subscribe_config";
pub const REGISTER_CONFIG_KEY: &str = "system:register_config";
pub const VERIFY_CONFIG_KEY: &str = "system:verify_config";
pub const EMAIL_SMTP_CONFIG_KEY: &str = "system:email_smtp_config";
pub const NODE_CONFIG_KEY: &str = "system:node_config";
pub const INVITE_CONFIG_KEY: &str = "system:invite_config";
pub const TELEGRAM_CONFIG_KEY: &str = "system:telegram_config";
pub const ADMIN_TELEGRAM_CHAT_IDS_KEY: &str = "system:telegram_admin_chat_ids";
pub const TOS_CONFIG_KEY: &str = "system:tos_config";
pub const VERIFY_CODE_CONFIG_KEY: &str = "system:verify_code_config";
pub const SESSION_ID_KEY: &str = "auth:session_id";
pub const GLOBAL_CONFIG_KEY: &str = "system:global_config";
pub const AUTH_CODE_CACHE_KEY: &str = "auth:verify:email";
pub const AUTH_CODE_TELEPHONE_CACHE_KEY: &str = "auth:verify:telephone";
pub const COMMON_STAT_CACHE_KEY: &str = "common:stat";
pub const SERVER_COUNT_CACHE_KEY: &str = "server:count";
pub const SEND_INTERVAL_KEY_PREFIX: &str = "send:interval:";
pub const SEND_COUNT_LIMIT_KEY_PREFIX: &str = "send:limit:";
