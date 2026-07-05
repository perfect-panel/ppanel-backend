use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::update_node_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_node(
    State(state): State<AppState>,
    Json(req): Json<UpdateNodeRequest>,
) -> HttpResult {
    match update_node_service::update_node(state.repos.node.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
