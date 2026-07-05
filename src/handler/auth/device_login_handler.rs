use axum::{extract::State, Extension, Json};

use crate::handler::AppState;
use crate::middleware::device_middleware::DeviceContext;
use crate::model::dto::auth::{DeviceLoginRequest, LoginResponse};
use crate::service::auth::device_login_service::DeviceLoginService;
use result::http_result::{build_http_result, HttpResult};

pub async fn device_login(
    State(state): State<AppState>,
    Extension(device): Extension<DeviceContext>,
    Json(mut req): Json<DeviceLoginRequest>,
) -> HttpResult {
    if !device.ip.is_empty() { req.ip = device.ip; }
    if !device.user_agent.is_empty() { req.user_agent = device.user_agent; }
    if !device.identifier.is_empty() { req.identifier = device.identifier; }

    let svc = DeviceLoginService::new(state.repos.clone(), state.config.clone(), state.cache.clone());
    match svc.login(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<LoginResponse>(None, Some(err)),
    }
}
