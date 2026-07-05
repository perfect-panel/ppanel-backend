use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::delete_user_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_user(
    State(state): State<AppState>,
    Json(req): Json<GetDetailRequest>,
) -> HttpResult {
    match delete_user_service::delete_user(&state.repos, req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
