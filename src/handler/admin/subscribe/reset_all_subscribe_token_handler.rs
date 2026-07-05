use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::subscribe::reset_all_subscribe_token_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn reset_all_subscribe_token(
    State(state): State<AppState>,
) -> HttpResult {
    match reset_all_subscribe_token_service::reset_all_subscribe_token(state.repos.subscribe.as_ref()).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
