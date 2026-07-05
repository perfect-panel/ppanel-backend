use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::create_user_subscribe_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_user_subscribe(
    State(state): State<AppState>,
    Json(req): Json<CreateUserSubscribeRequest>,
) -> HttpResult {
    match create_user_subscribe_service::create_user_subscribe(
        &state.repos,
        req.user_id,
        req.subscribe_id,
        req.expired_at, // maps to duration_days in service
        req.traffic,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
