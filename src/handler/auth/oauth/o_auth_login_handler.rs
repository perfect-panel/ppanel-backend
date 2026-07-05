use axum::{extract::State, Extension, Json};

use crate::handler::AppState;
use crate::middleware::device_middleware::DeviceContext;
use crate::model::dto::auth::{OAthLoginRequest, OAuthLoginResponse};
use crate::service::auth::oauth::o_auth_login_service::OAuthLoginService;
use result::http_result::{build_http_result, HttpResult};

pub async fn o_auth_login(
    State(state): State<AppState>,
    Extension(_device): Extension<DeviceContext>,
    Json(req): Json<OAthLoginRequest>,
) -> HttpResult {
    let svc = OAuthLoginService::new(state.repos.clone(), state.config.clone(), state.cache.clone());
    match svc.login(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<OAuthLoginResponse>(None, Some(err)),
    }
}
