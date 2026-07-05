/// Handler for POST /v1/server/push-online
/// Delegates to `push_online_users_service`.

use axum::{extract::State, Json};
use axum::http::HeaderMap;

use crate::handler::AppState;
use crate::model::dto::server::OnlineUsersRequest;
use crate::service::server::push_online_users_service;
use result::http_result::{build_http_result, HttpResult};

use super::get_server_config_handler::check_node_auth;

pub async fn push_online_users(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<OnlineUsersRequest>,
) -> HttpResult {
    if let Err(e) = check_node_auth(&headers, &state.config.node.node_secret, &req.common.secret_key) {
        return build_http_result::<()>(None, Some(e));
    }

    let result = push_online_users_service::push_online_users(
        state.repos.clone(),
        state.config.clone(),
        state.cache.clone(),
        req,
    )
    .await;

    match result {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
