//! Telegram webhook handler.

use axum::{body::Bytes, extract::State, response::{IntoResponse, Response}};
use axum::http::StatusCode;

use crate::handler::AppState;
use crate::service::telegram::telegram_service::TelegramService;

/// `POST /v1/telegram`
pub async fn telegram_handler(
    State(state): State<AppState>,
    body: Bytes,
) -> Response {
    let svc = TelegramService::new(state.repos.clone(), state.config.clone());
    match svc.handle_update(&body).await {
        Ok(Some(msg)) => {
            if let Err(e) = svc.send_message(&msg).await {
                tracing::warn!("telegram send_message failed: {e:#}");
            }
            (StatusCode::OK, "ok").into_response()
        }
        Ok(None) => (StatusCode::OK, "ok").into_response(),
        Err(e) => {
            tracing::error!("telegram handler error: {e:#}");
            (StatusCode::OK, "ok").into_response()
        }
    }
}
