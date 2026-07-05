use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::filter_server_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn filter_server_list(
    State(state): State<AppState>,
    Query(req): Query<FilterServerListRequest>,
) -> HttpResult {
    match filter_server_list_service::filter_server_list(state.repos.node.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
