use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::subscribe::subscribe_sort_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn subscribe_sort(
    State(state): State<AppState>,
    Json(req): Json<SubscribeSortRequest>,
) -> HttpResult {
    match subscribe_sort_service::subscribe_sort(state.repos.subscribe.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
