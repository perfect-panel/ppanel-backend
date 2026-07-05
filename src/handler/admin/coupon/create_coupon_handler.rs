use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::coupon::create_coupon_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_coupon(
    State(state): State<AppState>,
    Json(req): Json<CreateCouponRequest>,
) -> HttpResult {
    match create_coupon_service::create_coupon(state.repos.coupon.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
