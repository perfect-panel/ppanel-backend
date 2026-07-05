use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::batch_delete_user_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn batch_delete_user(
    State(state): State<AppState>,
    Json(req): Json<BatchDeleteUserRequest>,
) -> HttpResult {
    match batch_delete_user_service::batch_delete_user(&state.repos, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
