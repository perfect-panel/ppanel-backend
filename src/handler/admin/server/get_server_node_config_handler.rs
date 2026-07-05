use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::server::GetServerConfigRequest;
use crate::service::admin::server::get_server_node_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_server_node_config(
    State(state): State<AppState>,
    Query(req): Query<GetServerConfigRequest>,
) -> HttpResult {
    match get_server_node_config_service::get_server_node_config(state.repos.node.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
