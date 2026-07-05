//! Request logging middleware based on `tower-http::TraceLayer`.
//!
//! Replaces Go's `loggerMiddleware.go`.  Every inbound HTTP request gets a
//! tracing span with method / path / query; the response is logged at a level
//! that matches the Go convention:
//!
//! | Status range | tracing level |
//! |-------------|---------------|
//! | 500–599     | `error!`      |
//! | 404         | `debug!`      |
//! | everything  | `info!`       |
//!
//! Registered fields: `method`, `path`, `query`, `status`, `duration_ms`.
//! 5xx responses additionally log an `error` field if one is stored in the
//! response extensions.
use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, Response};
use tower_http::trace::{MakeSpan, OnResponse};
use tracing::Span;

// ─── span factory ───────────────────────────────────────────────────────────

/// Creates the per-request tracing span.
#[derive(Clone, Debug)]
pub struct RequestSpan;

impl MakeSpan<Body> for RequestSpan {
    fn make_span(&mut self, req: &Request<Body>) -> Span {
        let method = req.method().as_str();
        let path = req.uri().path();
        let query = req.uri().query().unwrap_or("");
        tracing::info_span!(
            "request",
            method  = %method,
            path    = %path,
            query   = %query,
            status  = tracing::field::Empty,
            duration_ms = tracing::field::Empty,
        )
    }
}

// ─── response hook ──────────────────────────────────────────────────────────

/// Logs the response once it is ready.
#[derive(Clone, Debug)]
pub struct RequestLog;

impl<B> OnResponse<B> for RequestLog {
    fn on_response(self, resp: &Response<B>, latency: Duration, span: &Span) {
        let status = resp.status().as_u16();
        let duration_ms = latency.as_secs_f64() * 1_000.0;

        span.record("status", status);
        span.record("duration_ms", duration_ms);

        if status >= 500 {
            // Attach error detail if the handler stored one in extensions.
            let err_msg = resp
                .extensions()
                .get::<String>()
                .map(|s| s.as_str())
                .unwrap_or("internal server error");
            tracing::error!(parent: span, error = %err_msg, "request failed");
        } else if status == 404 {
            tracing::debug!(parent: span, "not found");
        } else {
            tracing::info!(parent: span, "request completed");
        }
    }
}
