use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::system::pre_view_node_multiplier_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn pre_view_node_multiplier(
    State(state): State<AppState>,
) -> HttpResult {
    match pre_view_node_multiplier_service::pre_view_node_multiplier(&state.repos).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
