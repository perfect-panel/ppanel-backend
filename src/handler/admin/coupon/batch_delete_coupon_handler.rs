use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::coupon::batch_delete_coupon_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn batch_delete_coupon(
    State(state): State<AppState>,
    Json(req): Json<BatchDeleteCouponRequest>,
) -> HttpResult {
    match batch_delete_coupon_service::batch_delete_coupon(state.repos.coupon.as_ref(), req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
