use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::create_server_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_server(
    State(state): State<AppState>,
    Json(req): Json<CreateServerRequest>,
) -> HttpResult {
    match create_server_service::create_server(state.repos.node.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
