use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::coupon::get_coupon_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_coupon_list(
    State(state): State<AppState>,
    Query(req): Query<GetCouponListRequest>,
) -> HttpResult {
    match get_coupon_list_service::get_coupon_list(state.repos.coupon.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
