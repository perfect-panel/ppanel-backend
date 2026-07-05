use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::console::query_server_total_data_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_server_total_data(State(state): State<AppState>) -> HttpResult {
    match query_server_total_data_service::query_server_total_data(&state.repos).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
