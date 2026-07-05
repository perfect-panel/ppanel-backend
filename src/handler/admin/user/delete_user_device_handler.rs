use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::delete_user_device_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_user_device(
    State(state): State<AppState>,
    Json(req): Json<DeleteUserDeivceRequest>,
) -> HttpResult {
    match delete_user_device_service::delete_user_device(&state.repos, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
