use axum::extract::{Query, State};
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::order::query_order_detail_service::QueryOrderDetailService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_order_detail(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Query(req): Query<QueryOrderDetailRequest>,
) -> HttpResult {
    let svc = QueryOrderDetailService::new(state.repos.clone());
    match svc.query(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
