use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::public::subscribe::query_subscribe_list_service::QuerySubscribeListService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_subscribe_list(
    State(state): State<AppState>,
    Query(req): Query<QuerySubscribeListRequest>,
) -> HttpResult {
    let svc = QuerySubscribeListService::new(state.repos.clone());
    match svc.query_list(1, 100).await {
        Ok((_, list)) => build_http_result(Some(list), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
