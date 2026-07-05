use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::marketing::stop_batch_send_email_task_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn stop_batch_send_email_task(
    State(state): State<AppState>,
    Json(req): Json<StopBatchSendEmailTaskRequest>,
) -> HttpResult {
    match stop_batch_send_email_task_service::stop_batch_send_email_task(
        state.repos.task.as_ref(),
        req,
    )
    .await
    {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
