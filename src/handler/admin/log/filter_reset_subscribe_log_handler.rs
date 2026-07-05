use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::log::filter_reset_subscribe_log_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn filter_reset_subscribe_log(
    State(state): State<AppState>,
    Query(req): Query<FilterResetSubscribeLogRequest>,
) -> HttpResult {
    match filter_reset_subscribe_log_service::filter_reset_subscribe_log(state.repos.log.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
