use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::system::get_verify_code_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_verify_code_config(
    State(state): State<AppState>,
) -> HttpResult {
    match get_verify_code_config_service::get_verify_code_config(&state.repos).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
