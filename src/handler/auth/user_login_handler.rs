use axum::{extract::State, Extension, Json};

use crate::handler::AppState;
use crate::middleware::device_middleware::DeviceContext;
use crate::model::dto::auth::{LoginResponse, UserLoginRequest};
use crate::service::auth::user_login_service::UserLoginService;
use result::http_result::{build_http_result, HttpResult};

pub async fn user_login(
    State(state): State<AppState>,
    Extension(device): Extension<DeviceContext>,
    Json(mut req): Json<UserLoginRequest>,
) -> HttpResult {
    // Override transport fields from middleware context (headers take precedence)
    if !device.ip.is_empty() { req.ip = device.ip; }
    if !device.user_agent.is_empty() { req.user_agent = device.user_agent; }
    if !device.identifier.is_empty() { req.identifier = device.identifier; }
    if !device.login_type.is_empty() { req.login_type = device.login_type; }

    let svc = UserLoginService::new(state.repos.clone(), state.config.clone(), state.cache.clone());
    match svc.login(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<LoginResponse>(None, Some(err)),
    }
}
