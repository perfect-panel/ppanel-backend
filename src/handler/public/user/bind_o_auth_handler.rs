use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::bind_o_auth_service::BindOAuthService;
use result::http_result::{build_http_result, HttpResult};

pub async fn bind_o_auth(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<BindOAuthRequest>,
) -> HttpResult {
    let svc = BindOAuthService::new(state.repos.clone());
    match svc.bind_o_auth(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
