use axum::{extract::State, Extension, Json};

use crate::handler::AppState;
use crate::middleware::device_middleware::DeviceContext;
use crate::model::dto::auth::{LoginResponse, OAuthLoginGetTokenRequest};
use crate::service::auth::oauth::o_auth_login_get_token_service::OAuthLoginGetTokenService;
use result::http_result::{build_http_result, HttpResult};

pub async fn o_auth_login_get_token(
    State(state): State<AppState>,
    Extension(device): Extension<DeviceContext>,
    Json(req): Json<OAuthLoginGetTokenRequest>,
) -> HttpResult {
    let svc = OAuthLoginGetTokenService::new(
        state.repos.clone(), state.config.clone(), state.cache.clone(),
    );
    match svc.get_token(req, &device.ip, &device.user_agent).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<LoginResponse>(None, Some(err)),
    }
}
