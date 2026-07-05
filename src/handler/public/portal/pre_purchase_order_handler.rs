use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::public::portal::pre_purchase_order_service::PrePurchaseOrderService;
use result::http_result::{build_http_result, HttpResult};

pub async fn pre_purchase_order(
    State(state): State<AppState>,
    Json(req): Json<PrePurchaseOrderRequest>,
) -> HttpResult {
    let svc = PrePurchaseOrderService::new(state.repos.clone());
    match svc.pre_purchase(req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
