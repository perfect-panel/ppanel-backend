use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::public::portal::purchase_checkout_service::PurchaseCheckoutService;
use result::http_result::{build_http_result, HttpResult};

pub async fn purchase_checkout(
    State(state): State<AppState>,
    Json(req): Json<CheckoutOrderRequest>,
) -> HttpResult {
    let svc = PurchaseCheckoutService::new(state.repos.clone());
    match svc.checkout(req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
