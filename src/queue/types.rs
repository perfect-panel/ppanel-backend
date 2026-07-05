/// Task type constants, ported from `server/queue/types/*.go`.
///
/// Prefix convention (matching Go):
/// - `scheduler:` — periodic tasks registered by the scheduler
/// - `forthwith:` — tasks enqueued immediately by HTTP handlers
/// - `defer:`     — tasks enqueued with a delay
/// - `scheduled:` — tasks enqueued for a specific future time

// ── scheduler ──────────────────────────────────────────────────────────
pub const SCHEDULER_CHECK_SUBSCRIPTION: &str = "scheduler:check:subscription";
pub const SCHEDULER_TOTAL_SERVER_DATA: &str = "scheduler:total:server";
pub const SCHEDULER_RESET_TRAFFIC: &str = "scheduler:reset:traffic";
pub const SCHEDULER_TRAFFIC_STAT: &str = "scheduler:traffic:stat";

// ── order ──────────────────────────────────────────────────────────────
pub const FORTHWITH_ACTIVATE_ORDER: &str = "forthwith:activate:order";
pub const DEFER_CLOSE_ORDER: &str = "defer:close:order";

// ── email ──────────────────────────────────────────────────────────────
pub const FORTHWITH_SEND_EMAIL: &str = "forthwith:send:email";
pub const SCHEDULED_BATCH_SEND_EMAIL: &str = "scheduled:batch:send:email";

// ── sms ────────────────────────────────────────────────────────────────
pub const FORTHWITH_SEND_SMS: &str = "forthwith:sms:send";

// ── server / traffic ───────────────────────────────────────────────────
pub const FORTHWITH_TRAFFIC_STATISTICS: &str = "forthwith:traffic:statistics";

// ── task / quota ───────────────────────────────────────────────────────
pub const FORTHWITH_QUOTA_TASK: &str = "forthwith:quota:task";
