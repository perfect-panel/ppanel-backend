use axum::extract::{Query, State};
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::get_login_log_service::GetLoginLogService;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_login_log(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Query(req): Query<GetLoginLogRequest>,
) -> HttpResult {
    let svc = GetLoginLogService::new(state.repos.clone());
    match svc.get_login_log(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
