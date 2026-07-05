use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::marketing::query_quota_task_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_quota_task_list(
    State(state): State<AppState>,
    Query(req): Query<QueryQuotaTaskListRequest>,
) -> HttpResult {
    match query_quota_task_list_service::query_quota_task_list(state.repos.task.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
