use axum::extract::State;
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::service::admin::user::current_user_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn current_user(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
) -> HttpResult {
    match current_user_service::current_user(&state.repos, auth.user_id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
