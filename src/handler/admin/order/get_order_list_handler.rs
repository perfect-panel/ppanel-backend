use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::order::get_order_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_order_list(
    State(state): State<AppState>,
    Query(req): Query<GetOrderListRequest>,
) -> HttpResult {
    match get_order_list_service::get_order_list(state.repos.order.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
