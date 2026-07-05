use axum::extract::{Query, State};
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::server::GetServerConfigRequest;
use crate::model::entity::node::ServerConfigOverride;
use crate::service::admin::server::update_server_node_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_server_node_config(
    State(state): State<AppState>,
    Query(req): Query<GetServerConfigRequest>,
    Json(body): Json<ServerConfigOverride>,
) -> HttpResult {
    match update_server_node_config_service::update_server_node_config(state.repos.node.as_ref(), req, body).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
