use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::unbind_device_service::UnbindDeviceService;
use result::http_result::{build_http_result, HttpResult};

pub async fn unbind_device(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<UnbindDeviceRequest>,
) -> HttpResult {
    let svc = UnbindDeviceService::new(state.repos.clone());
    match svc.unbind_device(auth.user_id, req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
