use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::announcement::get_announcement_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_announcement_list(
    State(state): State<AppState>,
    Query(req): Query<GetAnnouncementListRequest>,
) -> HttpResult {
    match get_announcement_list_service::get_announcement_list(state.repos.announcement.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
