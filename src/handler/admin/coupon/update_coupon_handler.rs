use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::coupon::update_coupon_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_coupon(
    State(state): State<AppState>,
    Json(req): Json<UpdateCouponRequest>,
) -> HttpResult {
    match update_coupon_service::update_coupon(state.repos.coupon.as_ref(), req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
