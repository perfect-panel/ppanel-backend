//! Subscribe endpoint handlers.

use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use axum::http::{header, StatusCode};

use crate::handler::AppState;
use crate::service::subscribe::subscribe_service::SubscribeService;

/// `GET /v1/subscribe/config/:token`
pub async fn subscribe_handler(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let ua = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let host = headers
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("localhost")
        .to_string();

    let svc = SubscribeService::new(state.repos.clone(), state.config.clone());
    match svc.handle_subscribe(&ua, &token, &host, params).await {
        Ok(out) => {
            let mut resp_headers = HeaderMap::new();
            if let Ok(v) = out.content_type.parse() {
                resp_headers.insert(header::CONTENT_TYPE, v);
            }
            if let Ok(v) = out.userinfo.parse() {
                resp_headers.insert("Subscription-Userinfo", v);
            }
            if let Ok(v) = out.disposition.parse() {
                resp_headers.insert(header::CONTENT_DISPOSITION, v);
            }
            (StatusCode::OK, resp_headers, out.content).into_response()
        }
        Err(e) => {
            tracing::error!("subscribe error: {e:#}");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

/// `GET /v1/subscribe/pan/:token` — pan-domain subscribe variant.
pub async fn pan_domain_subscribe_handler(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    subscribe_handler(State(state), Path(token), Query(params), headers).await
}
