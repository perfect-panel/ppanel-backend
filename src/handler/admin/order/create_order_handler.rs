use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::order::create_order_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> HttpResult {
    match create_order_service::create_order(state.repos.order.as_ref(), req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
