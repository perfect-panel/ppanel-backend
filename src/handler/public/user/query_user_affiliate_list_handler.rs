use axum::extract::{Query, State};
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::query_user_affiliate_list_service::QueryUserAffiliateListService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_user_affiliate_list(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Query(req): Query<QueryUserAffiliateListRequest>,
) -> HttpResult {
    let svc = QueryUserAffiliateListService::new(state.repos.clone());
    match svc.query_user_affiliate_list(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
