use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::model::entity::user::Device;
use crate::service::admin::user::update_user_device_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_user_device(
    State(state): State<AppState>,
    Json(req): Json<UserDevice>,
) -> HttpResult {
    let device = Device {
        id: req.id,
        ip: req.ip,
        user_id: 0, // not carried in DTO; service updates by id only
        user_agent: if req.user_agent.is_empty() { None } else { Some(req.user_agent) },
        identifier: req.identifier,
        online: req.online,
        enabled: req.enabled,
        created_at: req.created_at,
        updated_at: req.updated_at,
    };
    match update_user_device_service::update_user_device(&state.repos, device).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
