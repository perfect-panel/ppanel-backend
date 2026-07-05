//! Server auth middleware — port of `serverMiddleware.go`.
//!
//! Validates the `secret_key` query param against `config.node.node_secret`.

use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use crate::handler::AppState;

#[derive(Deserialize)]
struct SecretKeyQuery {
    secret_key: Option<String>,
}

pub async fn server_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let secret = req
        .uri()
        .query()
        .and_then(|q| serde_urlencoded::from_str::<SecretKeyQuery>(q).ok())
        .and_then(|q| q.secret_key);

    match secret {
        Some(key) if key == state.config.node.node_secret => next.run(req).await,
        _ => (StatusCode::FORBIDDEN, "Forbidden").into_response(),
    }
}
