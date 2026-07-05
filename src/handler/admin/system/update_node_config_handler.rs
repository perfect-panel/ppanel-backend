use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::system::update_node_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_node_config(
    State(state): State<AppState>,
    Json(req): Json<NodeConfig>,
) -> HttpResult {
    match update_node_config_service::update_node_config(&state.repos, req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
