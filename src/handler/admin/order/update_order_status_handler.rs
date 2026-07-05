use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::order::update_order_status_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_order_status(
    State(state): State<AppState>,
    Json(req): Json<UpdateOrderStatusRequest>,
) -> HttpResult {
    match update_order_status_service::update_order_status(state.repos.order.as_ref(), req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
