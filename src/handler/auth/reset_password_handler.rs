use axum::{extract::State, Extension, Json};

use crate::handler::AppState;
use crate::middleware::device_middleware::DeviceContext;
use crate::model::dto::auth::{LoginResponse, ResetPasswordRequest};
use crate::service::auth::reset_password_service::ResetPasswordService;
use result::http_result::{build_http_result, HttpResult};

pub async fn reset_password(
    State(state): State<AppState>,
    Extension(device): Extension<DeviceContext>,
    Json(mut req): Json<ResetPasswordRequest>,
) -> HttpResult {
    if !device.ip.is_empty() { req.ip = device.ip; }
    if !device.user_agent.is_empty() { req.user_agent = device.user_agent; }
    if !device.identifier.is_empty() { req.identifier = device.identifier; }
    if !device.login_type.is_empty() { req.login_type = device.login_type; }

    let svc = ResetPasswordService::new(state.repos.clone(), state.config.clone(), state.cache.clone());
    match svc.reset(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<LoginResponse>(None, Some(err)),
    }
}
