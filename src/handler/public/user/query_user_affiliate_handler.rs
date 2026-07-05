use axum::extract::State;
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::service::public::user::query_user_affiliate_service::QueryUserAffiliateService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_user_affiliate(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
) -> HttpResult {
    let svc = QueryUserAffiliateService::new(state.repos.clone());
    match svc.query_user_affiliate(auth.user_id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
