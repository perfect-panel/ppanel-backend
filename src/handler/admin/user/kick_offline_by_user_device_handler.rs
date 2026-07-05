use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::kick_offline_by_user_device_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn kick_offline_by_user_device(
    State(state): State<AppState>,
    Json(req): Json<KickOfflineRequest>,
) -> HttpResult {
    match kick_offline_by_user_device_service::kick_offline_by_user_device(
        &state.repos,
        &state.cache,
        req,
    )
    .await
    {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
