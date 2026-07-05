use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::application::delete_subscribe_application_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_subscribe_application(
    State(state): State<AppState>,
    Json(req): Json<DeleteSubscribeApplicationRequest>,
) -> HttpResult {
    match delete_subscribe_application_service::delete_subscribe_application(
        state.repos.client.as_ref(),
        req.id,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
