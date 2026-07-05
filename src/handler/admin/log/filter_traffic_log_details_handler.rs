use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::model::dto::log::FilterTrafficLogDetailsRequest;
use crate::service::admin::log::filter_balance_log_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn filter_traffic_log_details(
    State(state): State<AppState>,
    Query(req): Query<FilterTrafficLogDetailsRequest>,
) -> HttpResult {
    match filter_balance_log_service::filter_traffic_log_details(state.repos.log.as_ref(), req).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
