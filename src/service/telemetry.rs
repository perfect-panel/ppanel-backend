/// Telemetry facade — business audit log writer.
///
/// Wraps all 14 `system_logs` write paths so service code never touches
/// `LogRepo` or JSON serialisation directly.  Every method:
///   - Accepts plain Rust types (no JSON, no raw strings for enum values).
///   - Auto-fills `date` (YYYY-MM-DD UTC) and `created_at` (ms epoch).
///   - Calls `repos.log.insert()` and silently swallows errors via
///     `tracing::error!` so that a log failure never aborts the main flow.
use std::sync::Arc;

use chrono::Utc;

use crate::model::entity::log::{
    Balance, Commission, Gift, Login, LogType, Message, Register, ResetSubscribe,
    ServerTraffic, ServerTrafficRank, SubscribeLog, SystemLog, TrafficStat, UserTraffic,
    UserTrafficRank,
};
use crate::repository::Repositories;

pub struct Telemetry;

// ─── helpers ────────────────────────────────────────────────────────────────

fn today() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

async fn write(repos: &Arc<Repositories>, type_: LogType, object_id: i64, content: String) {
    let log = SystemLog {
        id: 0,
        type_: type_.0,
        date: Some(today()),
        object_id,
        content,
        created_at: now_ms(),
    };
    if let Err(e) = repos.log.insert(&log).await {
        tracing::error!(?e, log_type = type_.0, object_id, "failed to write business audit log");
    }
}

// ─── P0 — login / register ──────────────────────────────────────────────────

impl Telemetry {
    /// Record a login attempt (success or failure).
    ///
    /// Go counterpart: `Store.Log().Insert(ctx, &SystemLog{Type: LogTypeLogin, ...})`
    /// in `userLoginLogic`, `deviceLoginLogic`, `oAuthLogin`.
    pub async fn login(
        repos: &Arc<Repositories>,
        user_id: i64,
        method: &str,
        login_ip: &str,
        user_agent: &str,
        success: bool,
    ) {
        let content = Login {
            method: method.to_string(),
            login_ip: login_ip.to_string(),
            user_agent: user_agent.to_string(),
            success,
            timestamp: now_ms(),
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::LOGIN, user_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::login serialisation failed"),
        }
    }

    /// Record a successful registration.
    ///
    /// Go counterpart: `userRegisterLogic`, `telephoneRegister`.
    pub async fn register(
        repos: &Arc<Repositories>,
        user_id: i64,
        auth_method: &str,
        identifier: &str,
        register_ip: &str,
        user_agent: &str,
    ) {
        let content = Register {
            auth_method: auth_method.to_string(),
            identifier: identifier.to_string(),
            register_ip: register_ip.to_string(),
            user_agent: user_agent.to_string(),
            timestamp: now_ms(),
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::REGISTER, user_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::register serialisation failed"),
        }
    }
}

// ─── P1 — balance / commission / gift / subscribe_access ────────────────────

impl Telemetry {
    /// Record a balance change (recharge, withdraw, payment, refund, reward, adjust).
    ///
    /// `type_` should be one of the `BALANCE_TYPE_*` constants from `model::entity::log`.
    /// Go counterpart: `activateOrderLogic`, `purchaseLogic`, `renewalLogic`.
    pub async fn balance(
        repos: &Arc<Repositories>,
        user_id: i64,
        type_: i32,
        amount: i64,
        order_no: Option<String>,
        balance: i64,
    ) {
        let content = Balance {
            type_,
            amount,
            order_no,
            balance,
            timestamp: now_ms(),
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::BALANCE, user_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::balance serialisation failed"),
        }
    }

    /// Record a commission change.
    ///
    /// `type_` should be one of the `COMMISSION_TYPE_*` constants.
    /// Go counterpart: `activateOrderLogic`, `commissionWithdrawLogic`.
    pub async fn commission(
        repos: &Arc<Repositories>,
        user_id: i64,
        type_: i32,
        amount: i64,
        order_no: &str,
    ) {
        let content = Commission {
            type_,
            amount,
            order_no: order_no.to_string(),
            timestamp: now_ms(),
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::COMMISSION, user_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::commission serialisation failed"),
        }
    }

    /// Record a gift (bonus quota) change.
    ///
    /// `type_` should be one of the `GIFT_TYPE_*` constants.
    /// Go counterpart: `purchaseLogic`, `closeOrderLogic`, `resetTrafficLogic`.
    pub async fn gift(
        repos: &Arc<Repositories>,
        user_id: i64,
        type_: i32,
        order_no: &str,
        subscribe_id: i64,
        amount: i64,
        balance: i64,
        remark: Option<String>,
    ) {
        let content = Gift {
            type_,
            order_no: order_no.to_string(),
            subscribe_id,
            amount,
            balance,
            remark,
            timestamp: now_ms(),
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::GIFT, user_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::gift serialisation failed"),
        }
    }

    /// Record a subscribe config access (user fetched their proxy config).
    ///
    /// Go counterpart: `subscribeLogic`.
    pub async fn subscribe_access(
        repos: &Arc<Repositories>,
        user_subscribe_id: i64,
        token: &str,
        user_agent: &str,
        client_ip: &str,
    ) {
        let content = SubscribeLog {
            token: token.to_string(),
            user_agent: user_agent.to_string(),
            client_ip: client_ip.to_string(),
            user_subscribe_id,
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::SUBSCRIBE, user_subscribe_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::subscribe_access serialisation failed"),
        }
    }
}

// ─── P2 — traffic / email / sms / reset_subscribe ───────────────────────────

impl Telemetry {
    /// Record per-user-subscribe traffic statistics for a time window.
    ///
    /// Go counterpart: `trafficStatLogic`.
    pub async fn subscribe_traffic(
        repos: &Arc<Repositories>,
        user_subscribe_id: i64,
        download: i64,
        upload: i64,
    ) {
        let content = UserTraffic {
            subscribe_id: user_subscribe_id,
            user_id: 0, // filled by caller if available
            upload,
            download,
            total: upload + download,
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::SUBSCRIBE_TRAFFIC, user_subscribe_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::subscribe_traffic serialisation failed"),
        }
    }

    /// Record per-server traffic statistics for a time window.
    ///
    /// Go counterpart: `trafficStatLogic`.
    pub async fn server_traffic(
        repos: &Arc<Repositories>,
        server_id: i64,
        download: i64,
        upload: i64,
    ) {
        let content = ServerTraffic {
            server_id,
            upload,
            download,
            total: upload + download,
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::SERVER_TRAFFIC, server_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::server_traffic serialisation failed"),
        }
    }

    /// Record a subscribe traffic reset event.
    ///
    /// `type_` should be one of the `RESET_SUBSCRIBE_TYPE_*` constants.
    /// Go counterpart: `resetTrafficLogic`, `activateOrderLogic`.
    pub async fn reset_subscribe(
        repos: &Arc<Repositories>,
        user_id: i64,
        type_: i32,
        order_no: Option<String>,
    ) {
        let content = ResetSubscribe {
            type_,
            user_id,
            order_no,
            timestamp: now_ms(),
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::RESET_SUBSCRIBE, user_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::reset_subscribe serialisation failed"),
        }
    }

    /// Record an outbound email send attempt.
    ///
    /// Go counterpart: `sendEmailLogic`.
    pub async fn email_message(
        repos: &Arc<Repositories>,
        object_id: i64,
        to: &str,
        subject: Option<String>,
        content_json: serde_json::Value,
        platform: &str,
        template: &str,
        status: i16,
    ) {
        let content = Message {
            to: to.to_string(),
            subject,
            content: content_json,
            platform: platform.to_string(),
            template: template.to_string(),
            status,
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::EMAIL_MESSAGE, object_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::email_message serialisation failed"),
        }
    }

    /// Record an outbound SMS send attempt.
    ///
    /// Go counterpart: `sendSmsLogic`.
    pub async fn mobile_message(
        repos: &Arc<Repositories>,
        object_id: i64,
        to: &str,
        content_json: serde_json::Value,
        platform: &str,
        template: &str,
        status: i16,
    ) {
        let content = Message {
            to: to.to_string(),
            subject: None,
            content: content_json,
            platform: platform.to_string(),
            template: template.to_string(),
            status,
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::MOBILE_MESSAGE, object_id, json).await,
            Err(e) => tracing::error!(?e, "telemetry::mobile_message serialisation failed"),
        }
    }
}

// ─── P3 — traffic rankings / stats ──────────────────────────────────────────

impl Telemetry {
    /// Record the daily/periodic user traffic ranking snapshot.
    ///
    /// Go counterpart: `trafficStatLogic`.
    pub async fn user_traffic_rank(
        repos: &Arc<Repositories>,
        rank: std::collections::HashMap<u8, UserTraffic>,
    ) {
        let content = UserTrafficRank { rank };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::USER_TRAFFIC_RANK, 0, json).await,
            Err(e) => tracing::error!(?e, "telemetry::user_traffic_rank serialisation failed"),
        }
    }

    /// Record the daily/periodic server traffic ranking snapshot.
    ///
    /// Go counterpart: `trafficStatLogic`.
    pub async fn server_traffic_rank(
        repos: &Arc<Repositories>,
        rank: std::collections::HashMap<u8, ServerTraffic>,
    ) {
        let content = ServerTrafficRank { rank };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::SERVER_TRAFFIC_RANK, 0, json).await,
            Err(e) => tracing::error!(?e, "telemetry::server_traffic_rank serialisation failed"),
        }
    }

    /// Record the aggregated traffic summary for a period.
    ///
    /// Go counterpart: `trafficStatLogic`.
    pub async fn traffic_stat(
        repos: &Arc<Repositories>,
        upload: i64,
        download: i64,
    ) {
        let content = TrafficStat {
            upload,
            download,
            total: upload + download,
        };
        match serde_json::to_string(&content) {
            Ok(json) => write(repos, LogType::TRAFFIC_STAT, 0, json).await,
            Err(e) => tracing::error!(?e, "telemetry::traffic_stat serialisation failed"),
        }
    }
}
