use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::server::get_server_protocols_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_server_protocols(
    State(state): State<AppState>,
    Query(req): Query<GetServerProtocolsRequest>,
) -> HttpResult {
    match get_server_protocols_service::get_server_protocols(state.repos.node.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
