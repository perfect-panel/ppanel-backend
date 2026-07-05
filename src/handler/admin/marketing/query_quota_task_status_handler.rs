use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::marketing::query_quota_task_status_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_quota_task_status(
    State(state): State<AppState>,
    Json(req): Json<QueryQuotaTaskStatusRequest>,
) -> HttpResult {
    match query_quota_task_status_service::query_quota_task_status(state.repos.task.as_ref(), req)
        .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
