use axum::extract::{Query, State};
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::order::QueryOrderListRequest;
use crate::service::public::portal::query_purchase_order_service::QueryPurchaseOrderService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_purchase_order(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Query(req): Query<QueryOrderListRequest>,
) -> HttpResult {
    let svc = QueryPurchaseOrderService::new(state.repos.clone());
    match svc.query(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
