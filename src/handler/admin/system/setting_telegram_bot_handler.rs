use axum::extract::State;
use crate::handler::AppState;
use crate::service::admin::system::setting_telegram_bot_service::setting_telegram_bot;
use result::http_result::{build_http_result, HttpResult};

pub async fn setting_telegram_bot_handler(State(state): State<AppState>) -> HttpResult {
    match setting_telegram_bot(&state.config).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
