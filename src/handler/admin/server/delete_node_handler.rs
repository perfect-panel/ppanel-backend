use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::delete_node_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_node(
    State(state): State<AppState>,
    Json(req): Json<DeleteNodeRequest>,
) -> HttpResult {
    match delete_node_service::delete_node(state.repos.node.as_ref(), req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
