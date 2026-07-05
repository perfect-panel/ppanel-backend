use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::reset_user_subscribe_token_service::ResetUserSubscribeTokenService;
use result::http_result::{build_http_result, HttpResult};

pub async fn reset_user_subscribe_token(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<ResetUserSubscribeTokenRequest>,
) -> HttpResult {
    let svc = ResetUserSubscribeTokenService::new(state.repos.clone());
    match svc.reset_user_subscribe_token(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
