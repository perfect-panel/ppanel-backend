use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::log::filter_login_log_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn filter_login_log(
    State(state): State<AppState>,
    Query(req): Query<FilterLoginLogRequest>,
) -> HttpResult {
    match filter_login_log_service::filter_login_log(state.repos.log.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
