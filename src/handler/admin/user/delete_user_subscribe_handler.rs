use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::delete_user_subscribe_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_user_subscribe(
    State(state): State<AppState>,
    Json(req): Json<DeleteUserSubscribeRequest>,
) -> HttpResult {
    match delete_user_subscribe_service::delete_user_subscribe(&state.repos, req.user_subscribe_id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
