use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::update_bind_mobile_service::UpdateBindMobileService;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_bind_mobile(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<UpdateBindMobileRequest>,
) -> HttpResult {
    let svc = UpdateBindMobileService::new(state.repos.clone());
    match svc.update_bind_mobile(auth.user_id, req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
