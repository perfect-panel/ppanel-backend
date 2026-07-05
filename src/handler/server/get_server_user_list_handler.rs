/// Handler for GET /v1/server/user
/// Delegates to `get_server_user_list_service`.

use axum::{extract::State, Json};
use axum::http::HeaderMap;

use crate::handler::AppState;
use crate::model::dto::server::{GetServerUserListRequest, GetServerUserListResponse};
use crate::service::server::get_server_user_list_service;
use crate::service::server::meta::RequestMeta;
use result::http_result::{build_http_result, HttpResult};

use super::get_server_config_handler::check_node_auth;

pub async fn get_server_user_list(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<GetServerUserListRequest>,
) -> HttpResult {
    if let Err(e) = check_node_auth(&headers, &state.config.node.node_secret, &req.common.secret_key) {
        return build_http_result::<GetServerUserListResponse>(None, Some(e));
    }

    let if_none_match = headers
        .get("If-None-Match")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();

    let meta = RequestMeta { if_none_match };

    let result = get_server_user_list_service::get_server_user_list(
        state.repos.clone(),
        state.config.clone(),
        state.cache.clone(),
        req.common.server_id,
        &req.common.protocol,
        meta,
    )
    .await;

    match result {
        Ok((resp, _)) => build_http_result(Some(resp), None),
        Err(e) => build_http_result::<GetServerUserListResponse>(None, Some(e)),
    }
}
