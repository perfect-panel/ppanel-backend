use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::order::{PortalPurchaseRequest, PurchaseOrderRequest};
use crate::service::public::portal::purchase_service::PortalPurchaseService;
use result::http_result::{build_http_result, HttpResult};

pub async fn purchase(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<PortalPurchaseRequest>,
) -> HttpResult {
    let svc = PortalPurchaseService::new(state.repos.clone(), state.config.clone(), state.queue.clone());
    let purchase_req = PurchaseOrderRequest {
        subscribe_id: req.subscribe_id,
        quantity: req.quantity,
        payment: Some(req.payment),
        coupon: req.coupon,
    };
    match svc.purchase(auth.user_id, purchase_req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
