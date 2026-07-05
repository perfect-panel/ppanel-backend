use axum::extract::State;
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::service::public::user::get_o_auth_methods_service::GetOAuthMethodsService;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_o_auth_methods(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
) -> HttpResult {
    let svc = GetOAuthMethodsService::new(state.repos.clone());
    match svc.get_o_auth_methods(auth.user_id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
