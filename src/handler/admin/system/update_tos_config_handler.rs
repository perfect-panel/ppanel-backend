use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::system::update_tos_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_tos_config(
    State(state): State<AppState>,
    Json(req): Json<TosConfig>,
) -> HttpResult {
    match update_tos_config_service::update_tos_config(&state.repos, req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
