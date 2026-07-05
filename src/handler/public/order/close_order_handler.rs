use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::order::close_order_service::CloseOrderService;
use result::http_result::{build_http_result, HttpResult};

pub async fn close_order(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<CloseOrderRequest>,
) -> HttpResult {
    let svc = CloseOrderService::new(state.repos.clone());
    match svc.close(auth.user_id, req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
