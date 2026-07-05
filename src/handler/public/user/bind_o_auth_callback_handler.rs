use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::bind_o_auth_callback_service::BindOAuthCallbackService;
use result::http_result::{build_http_result, HttpResult};

pub async fn bind_o_auth_callback(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<BindOAuthCallbackRequest>,
) -> HttpResult {
    let svc = BindOAuthCallbackService::new(state.repos.clone());
    match svc.bind_o_auth_callback(auth.user_id, req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
