use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::announcement::update_announcement_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_announcement(
    State(state): State<AppState>,
    Json(req): Json<UpdateAnnouncementRequest>,
) -> HttpResult {
    match update_announcement_service::update_announcement(state.repos.announcement.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
