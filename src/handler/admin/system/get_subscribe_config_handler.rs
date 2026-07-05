use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::system::get_subscribe_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_subscribe_config(
    State(state): State<AppState>,
) -> HttpResult {
    match get_subscribe_config_service::get_subscribe_config(&state.repos).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
