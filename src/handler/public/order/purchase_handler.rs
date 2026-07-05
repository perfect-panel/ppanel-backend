use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::order::purchase_service::PurchaseService;
use result::http_result::{build_http_result, HttpResult};

pub async fn purchase(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<PurchaseOrderRequest>,
) -> HttpResult {
    let svc = PurchaseService::new(state.repos.clone(), state.config.clone(), state.queue.clone());
    match svc.purchase(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
