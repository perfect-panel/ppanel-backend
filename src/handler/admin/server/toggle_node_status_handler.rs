use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::toggle_node_status_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn toggle_node_status(
    State(state): State<AppState>,
    Json(req): Json<ToggleNodeStatusRequest>,
) -> HttpResult {
    match toggle_node_status_service::toggle_node_status(state.repos.node.as_ref(), req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
