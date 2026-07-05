use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::update_server_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_server(
    State(state): State<AppState>,
    Json(req): Json<UpdateServerRequest>,
) -> HttpResult {
    match update_server_service::update_server(state.repos.node.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
