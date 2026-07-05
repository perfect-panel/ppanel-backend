use axum::extract::State;
use axum::Json;
use crate::handler::AppState;
use crate::model::dto::log::LogSetting;
use crate::service::admin::log::filter_balance_log_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_log_setting(
    State(state): State<AppState>,
    Json(req): Json<LogSetting>,
) -> HttpResult {
    match filter_balance_log_service::update_log_setting(&state.config, req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
