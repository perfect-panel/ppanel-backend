use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::system::get_invite_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_invite_config(
    State(state): State<AppState>,
) -> HttpResult {
    match get_invite_config_service::get_invite_config(&state.repos).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
