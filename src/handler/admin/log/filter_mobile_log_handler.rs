use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::service::admin::log::filter_balance_log_service::{self, FilterEmailMobileLogRequest};
use result::http_result::{build_http_result, HttpResult};

pub async fn filter_mobile_log(
    State(state): State<AppState>,
    Query(req): Query<FilterEmailMobileLogRequest>,
) -> HttpResult {
    match filter_balance_log_service::filter_mobile_log(state.repos.log.as_ref(), req).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
