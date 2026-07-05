use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::delete_server_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_server(
    State(state): State<AppState>,
    Json(req): Json<DeleteServerRequest>,
) -> HttpResult {
    match delete_server_service::delete_server(state.repos.node.as_ref(), req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
