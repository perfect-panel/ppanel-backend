use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::order::recharge_service::RechargeService;
use result::http_result::{build_http_result, HttpResult};

pub async fn recharge(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<RechargeOrderRequest>,
) -> HttpResult {
    let svc = RechargeService::new(state.repos.clone(), state.queue.clone());
    match svc.recharge(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
