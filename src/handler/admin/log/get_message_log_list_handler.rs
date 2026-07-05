use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::model::dto::log::GetMessageLogListRequest;
use crate::service::admin::log::filter_balance_log_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_message_log_list(
    State(state): State<AppState>,
    Query(req): Query<GetMessageLogListRequest>,
) -> HttpResult {
    match filter_balance_log_service::get_message_log_list(state.repos.log.as_ref(), req).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
