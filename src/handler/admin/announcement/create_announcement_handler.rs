use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::announcement::create_announcement_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_announcement(
    State(state): State<AppState>,
    Json(req): Json<CreateAnnouncementRequest>,
) -> HttpResult {
    match create_announcement_service::create_announcement(state.repos.announcement.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
