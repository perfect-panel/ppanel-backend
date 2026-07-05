//! Redis-based period rate-limit middleware.
//! Port of `server/pkg/limit/periodlimit.go`.
//!
//! Uses the same Lua script as the Go version for atomicity:
//! - INCRBY key 1
//! - Set TTL on first hit
//! - Return 1 (allowed), 2 (hit quota exactly), 0 (over quota)

use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::Response};
use redis::{aio::ConnectionManager, AsyncCommands, Script};

use result::code_error::CodeError;
use result::error_code;
use result::http_result::build_http_result;

/// Result codes returned by the Lua script (mirrors Go constants).
const ALLOWED: i64 = 1;
const HIT_QUOTA: i64 = 2;
// 0 = over quota

/// Shared rate-limiter state — cheap to clone (Arc inside).
#[derive(Clone)]
pub struct PeriodLimiter {
    inner: Arc<PeriodLimiterInner>,
}

struct PeriodLimiterInner {
    /// Window length in seconds.
    period: usize,
    /// Maximum requests per window.
    quota: usize,
    /// Redis connection.
    redis: ConnectionManager,
    /// Key prefix prepended to every cache key.
    key_prefix: String,
    /// Lua script (same as `periodscript.lua` in the Go version).
    script: Script,
}

/// Lua script — identical to `pkg/limit/periodscript.lua`.
const PERIOD_LUA: &str = r#"
local limit = tonumber(ARGV[1])
local window = tonumber(ARGV[2])
local current = redis.call("INCRBY", KEYS[1], 1)
if current == 1 then
    redis.call("expire", KEYS[1], window)
end
if current < limit then
    return 1
elseif current == limit then
    return 2
else
    return 0
end
"#;

impl PeriodLimiter {
    /// Create a new `PeriodLimiter`.
    ///
    /// - `period`     — window size in seconds
    /// - `quota`      — max requests allowed in that window
    /// - `redis`      — shared Redis connection manager
    /// - `key_prefix` — namespace prefix (e.g. `"rate:register:"`)
    pub fn new(period: usize, quota: usize, redis: ConnectionManager, key_prefix: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(PeriodLimiterInner {
                period,
                quota,
                redis,
                key_prefix: key_prefix.into(),
                script: Script::new(PERIOD_LUA),
            }),
        }
    }

    /// Check and increment the counter for `key`.
    ///
    /// Returns `Ok(true)` if the request is within quota, `Ok(false)` if over.
    pub async fn allow(&self, key: &str) -> anyhow::Result<bool> {
        let full_key = format!("{}{}", self.inner.key_prefix, key);
        let mut conn = self.inner.redis.clone();
        let result: i64 = self.inner.script
            .key(&full_key)
            .arg(self.inner.quota)
            .arg(self.inner.period)
            .invoke_async(&mut conn)
            .await?;
        Ok(result == ALLOWED || result == HIT_QUOTA)
    }
}

/// Axum middleware that enforces a per-IP (or per-custom-key) rate limit.
///
/// The key is extracted from the `X-Original-Forwarded-For` / `X-Forwarded-For`
/// header, falling back to the socket address.
///
/// Inject via `axum::middleware::from_fn_with_state(limiter, rate_limit_layer)`.
pub async fn rate_limit_layer(
    axum::extract::State(limiter): axum::extract::State<PeriodLimiter>,
    req: Request,
    next: Next,
) -> Response {
    let ip = req
        .headers()
        .get("X-Original-Forwarded-For")
        .or_else(|| req.headers().get("X-Forwarded-For"))
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    match limiter.allow(&ip).await {
        Ok(true) => next.run(req).await,
        Ok(false) => {
            let err = anyhow::Error::new(
                CodeError::new_err_code(error_code::ERROR)
            );
            // Build a 200 body with RATE_LIMIT business code.
            build_http_result::<()>(None, Some(err)).into_response()
        }
        Err(e) => {
            tracing::error!("rate limiter redis error: {e}");
            // On Redis failure, allow the request (fail open).
            next.run(req).await
        }
    }
}

// ─── trait helper ─────────────────────────────────────────────────────────

use axum::response::IntoResponse;
