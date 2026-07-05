use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::service::admin::log::filter_balance_log_service;
use crate::model::dto::log::FilterSubscribeTrafficRequest;
use result::http_result::{build_http_result, HttpResult};

pub async fn filter_user_subscribe_traffic_log(
    State(state): State<AppState>,
    Query(req): Query<FilterSubscribeTrafficRequest>,
) -> HttpResult {
    match filter_balance_log_service::filter_user_subscribe_traffic_log(state.repos.log.as_ref(), req).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
