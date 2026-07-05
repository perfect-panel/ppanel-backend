use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::marketing::create_quota_task_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_quota_task(
    State(state): State<AppState>,
    Json(req): Json<CreateQuotaTaskRequest>,
) -> HttpResult {
    match create_quota_task_service::create_quota_task(state.repos.task.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
