use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::subscribe::update_subscribe_group_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_subscribe_group(
    State(state): State<AppState>,
    Json(req): Json<UpdateSubscribeGroupRequest>,
) -> HttpResult {
    match update_subscribe_group_service::update_subscribe_group(state.repos.subscribe.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
