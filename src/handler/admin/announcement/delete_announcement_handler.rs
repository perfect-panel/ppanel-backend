use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::announcement::delete_announcement_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_announcement(
    State(state): State<AppState>,
    Json(req): Json<DeleteAnnouncementRequest>,
) -> HttpResult {
    match delete_announcement_service::delete_announcement(state.repos.announcement.as_ref(), req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
