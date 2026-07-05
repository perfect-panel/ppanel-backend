//! Device context middleware.
//!
//! Extracts per-request metadata from HTTP headers and injects a
//! [`DeviceContext`] into the request extensions.  Every downstream handler
//! that needs the client IP, User-Agent, device identifier, or login type can
//! extract it with `Extension<DeviceContext>`.
//!
//! This middleware NEVER rejects a request — it only enriches it.
//! The Go equivalent is the device_middleware that sets these values from
//! request headers before passing to the handler.

use axum::{extract::Request, middleware::Next, response::Response};

/// Per-request device / client metadata, populated from HTTP headers.
#[derive(Debug, Clone, Default)]
pub struct DeviceContext {
    /// Client device identifier (e.g. fingerprint / push token).
    pub identifier: String,
    /// Client IP address (from X-Original-Forwarded-For or X-Forwarded-For).
    pub ip: String,
    /// Raw User-Agent header value.
    pub user_agent: String,
    /// Login type hint supplied by the client (e.g. "email", "device").
    pub login_type: String,
}

pub async fn device_middleware(mut request: Request, next: Next) -> Response {
    let headers = request.headers();

    let ip = headers
        .get("X-Original-Forwarded-For")
        .or_else(|| headers.get("X-Forwarded-For"))
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let user_agent = headers
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let login_type = headers
        .get("Login-Type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let identifier = headers
        .get("Identifier")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let ctx = DeviceContext { identifier, ip, user_agent, login_type };
    request.extensions_mut().insert(ctx);

    next.run(request).await
}
