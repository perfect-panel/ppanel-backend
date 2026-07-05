use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::announcement::get_announcement_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_announcement(
    State(state): State<AppState>,
    Query(req): Query<GetAnnouncementRequest>,
) -> HttpResult {
    match get_announcement_service::get_announcement(state.repos.announcement.as_ref(), req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
