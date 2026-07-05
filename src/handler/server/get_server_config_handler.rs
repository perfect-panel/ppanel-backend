/// Handler for GET /v1/server/config
/// Delegates to `get_server_config_service`.

use axum::{extract::State, Json};
use axum::http::HeaderMap;

use crate::handler::AppState;
use crate::model::dto::server::{GetServerConfigRequest, GetServerConfigResponse};
use crate::service::server::get_server_config_service;
use crate::service::server::meta::RequestMeta;
use result::code_error::CodeError;
use result::error_code;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_server_config(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<GetServerConfigRequest>,
) -> HttpResult {
    if let Err(e) = check_node_auth(&headers, &state.config.node.node_secret, &req.common.secret_key) {
        return build_http_result::<GetServerConfigResponse>(None, Some(e));
    }

    let if_none_match = headers
        .get("If-None-Match")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();

    let meta = RequestMeta { if_none_match };

    let result = get_server_config_service::get_server_config(
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
        Err(e) => build_http_result::<GetServerConfigResponse>(None, Some(e)),
    }
}

pub fn check_node_auth(
    headers: &HeaderMap,
    node_secret: &str,
    req_secret: &str,
) -> Result<(), anyhow::Error> {
    let header_secret = headers
        .get("X-Node-Token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let provided = if !header_secret.is_empty() {
        header_secret
    } else {
        req_secret
    };

    if provided != node_secret {
        return Err(anyhow::anyhow!(CodeError::new_err_code_msg(
            error_code::INVALID_ACCESS,
            "invalid node token"
        )));
    }
    Ok(())
}
