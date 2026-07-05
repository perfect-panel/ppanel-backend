use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::model::dto::subscribe::QuerySubscribeListRequest;
use crate::service::public::announcement::query_announcement_service::QueryAnnouncementService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_announcement(State(state): State<AppState>) -> HttpResult {
    let svc = QueryAnnouncementService::new(state.repos.clone());
    match svc.query_list(1, 100).await {
        Ok((_, list)) => build_http_result(Some(list), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
