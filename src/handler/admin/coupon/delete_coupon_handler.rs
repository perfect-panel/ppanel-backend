use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::coupon::delete_coupon_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_coupon(
    State(state): State<AppState>,
    Json(req): Json<DeleteCouponRequest>,
) -> HttpResult {
    match delete_coupon_service::delete_coupon(state.repos.coupon.as_ref(), req.id).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
