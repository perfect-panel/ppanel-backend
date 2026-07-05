use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::toggle_user_subscribe_status_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn toggle_user_subscribe_status(
    State(state): State<AppState>,
    Json(req): Json<ToggleUserSubscribeStatusRequest>,
) -> HttpResult {
    match toggle_user_subscribe_status_service::toggle_user_subscribe_status(
        &state.repos,
        req.user_subscribe_id,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
