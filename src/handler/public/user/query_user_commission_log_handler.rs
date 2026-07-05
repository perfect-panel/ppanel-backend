use axum::extract::{Query, State};
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::log::FilterCommissionLogRequest;
use crate::service::public::user::query_user_commission_log_service::QueryUserCommissionLogService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_user_commission_log(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Query(req): Query<FilterCommissionLogRequest>,
) -> HttpResult {
    let svc = QueryUserCommissionLogService::new(state.repos.clone());
    match svc.query_user_commission_log(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
