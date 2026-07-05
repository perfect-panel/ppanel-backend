use axum::extract::State;
use crate::handler::AppState;
use crate::service::public::subscribe::query_subscribe_group_list_service::QuerySubscribeGroupListService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_subscribe_group_list(State(state): State<AppState>) -> HttpResult {
    let svc = QuerySubscribeGroupListService::new(state.repos.clone());
    match svc.query_list().await {
        Ok((_, list)) => build_http_result(Some(list), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
