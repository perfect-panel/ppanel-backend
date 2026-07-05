use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::marketing::get_batch_send_email_task_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_batch_send_email_task_list(
    State(state): State<AppState>,
    Query(req): Query<GetBatchSendEmailTaskListRequest>,
) -> HttpResult {
    match get_batch_send_email_task_list_service::get_batch_send_email_task_list(
        state.repos.task.as_ref(),
        req,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
