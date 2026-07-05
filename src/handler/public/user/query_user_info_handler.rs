use axum::extract::State;
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::service::public::user::query_user_info_service::QueryUserInfoService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_user_info(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
) -> HttpResult {
    let svc = QueryUserInfoService::new(state.repos.clone());
    match svc.query_user_info(auth.user_id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
