use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::reset_user_subscribe_traffic_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn reset_user_subscribe_traffic(
    State(state): State<AppState>,
    Json(req): Json<ResetUserSubscribeTrafficRequest>,
) -> HttpResult {
    match reset_user_subscribe_traffic_service::reset_user_subscribe_traffic(
        &state.repos,
        req.user_subscribe_id,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
