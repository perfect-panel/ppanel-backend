//! Request tracing middleware — port of `traceMiddleware.go`.
//! Uses tracing spans (no OpenTelemetry dependency added).

use axum::{extract::Request, middleware::Next, response::Response};
use tracing::Instrument;

pub async fn trace_middleware(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let span = tracing::info_span!("http_request", method = %method, path = %path);
    next.run(req).instrument(span).await
}
