use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::server::query_node_tag_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_node_tag(
    State(state): State<AppState>,
) -> HttpResult {
    match query_node_tag_service::query_node_tag(state.repos.node.as_ref()).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
