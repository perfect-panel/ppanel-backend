use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::service::admin::server::filter_node_list_service::{self, FilterNodeListRequest};
use result::http_result::{build_http_result, HttpResult};

pub async fn filter_node_list(
    State(state): State<AppState>,
    Query(req): Query<FilterNodeListRequest>,
) -> HttpResult {
    match filter_node_list_service::filter_node_list(state.repos.node.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
