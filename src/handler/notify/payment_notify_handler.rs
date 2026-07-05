//! Payment notification handlers for Alipay, EPay, and Stripe.

use std::collections::HashMap;

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use axum::http::StatusCode;

use crate::handler::AppState;
use crate::service::notify::alipay_notify_service::AlipayNotifyService;
use crate::service::notify::e_pay_notify_service::EPayNotifyService;
use crate::service::notify::stripe_notify_service::StripeNotifyService;

/// `POST /v1/notify/alipay/:token`
pub async fn alipay_notify_handler(
    State(state): State<AppState>,
    Path(token): Path<String>,
    body: Bytes,
) -> Response {
    let svc = AlipayNotifyService::new(state.repos.clone());
    match svc.handle(&token, body, &state.queue).await {
        Ok(msg) => (StatusCode::OK, msg).into_response(),
        Err(e) => {
            tracing::error!("alipay notify error: {e:#}");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

/// `GET /v1/notify/epay/:token` or `POST /v1/notify/epay/:token`
pub async fn epay_notify_handler(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let svc = EPayNotifyService::new(state.repos.clone());
    match svc.handle(&token, params, &state.queue).await {
        Ok(msg) => (StatusCode::OK, msg).into_response(),
        Err(e) => {
            tracing::error!("epay notify error: {e:#}");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

/// `POST /v1/notify/stripe/:token`
pub async fn stripe_notify_handler(
    State(state): State<AppState>,
    Path(token): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let signature = headers
        .get("Stripe-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let svc = StripeNotifyService::new(state.repos.clone());
    match svc.handle(&token, body, signature, &state.queue).await {
        Ok(msg) => (StatusCode::OK, msg).into_response(),
        Err(e) => {
            tracing::error!("stripe notify error: {e:#}");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}
