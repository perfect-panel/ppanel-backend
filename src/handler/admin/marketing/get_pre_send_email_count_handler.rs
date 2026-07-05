use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::marketing::get_pre_send_email_count_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_pre_send_email_count(
    State(state): State<AppState>,
    Json(req): Json<GetPreSendEmailCountRequest>,
) -> HttpResult {
    match get_pre_send_email_count_service::get_pre_send_email_count(
        state.repos.user.as_ref(),
        req,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
