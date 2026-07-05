use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::system::set_node_multiplier_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn set_node_multiplier(
    State(state): State<AppState>,
    Json(req): Json<SetNodeMultiplierRequest>,
) -> HttpResult {
    match set_node_multiplier_service::set_node_multiplier(&state.repos, req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
