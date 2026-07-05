use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::application::create_subscribe_application_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_subscribe_application(
    State(state): State<AppState>,
    Json(req): Json<CreateSubscribeApplicationRequest>,
) -> HttpResult {
    match create_subscribe_application_service::create_subscribe_application(
        state.repos.client.as_ref(),
        req,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
