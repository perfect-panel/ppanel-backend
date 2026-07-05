use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::marketing::get_batch_send_email_task_status_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_batch_send_email_task_status(
    State(state): State<AppState>,
    Json(req): Json<GetBatchSendEmailTaskStatusRequest>,
) -> HttpResult {
    match get_batch_send_email_task_status_service::get_batch_send_email_task_status(
        state.repos.task.as_ref(),
        req,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
