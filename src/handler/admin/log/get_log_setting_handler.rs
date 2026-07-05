use axum::extract::State;
use crate::handler::AppState;
use crate::service::admin::log::filter_balance_log_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_log_setting(
    State(state): State<AppState>,
) -> HttpResult {
    match filter_balance_log_service::get_log_setting(&state.config).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
