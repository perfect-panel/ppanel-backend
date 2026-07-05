/// Handler for POST /v1/server/query-config
/// Delegates to `query_server_protocol_config_service`.

use axum::{extract::State, Json};
use axum::http::HeaderMap;

use crate::handler::AppState;
use crate::model::dto::server::{QueryServerConfigRequest, QueryServerConfigResponse};
use crate::service::server::query_server_protocol_config_service;
use result::http_result::{build_http_result, HttpResult};

use super::get_server_config_handler::check_node_auth;

pub async fn query_server_protocol_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<QueryServerConfigRequest>,
) -> HttpResult {
    if let Err(e) = check_node_auth(&headers, &state.config.node.node_secret, &req.secret_key) {
        return build_http_result::<QueryServerConfigResponse>(None, Some(e));
    }

    let result = query_server_protocol_config_service::query_server_protocol_config(
        state.repos.clone(),
        state.config.clone(),
        req.server_id,
        req.protocols,
    )
    .await;

    match result {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(e) => build_http_result::<QueryServerConfigResponse>(None, Some(e)),
    }
}
