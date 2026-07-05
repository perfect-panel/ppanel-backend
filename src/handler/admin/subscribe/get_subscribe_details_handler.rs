use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::subscribe::get_subscribe_details_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_subscribe_details(
    State(state): State<AppState>,
    Query(req): Query<GetSubscribeDetailsRequest>,
) -> HttpResult {
    match get_subscribe_details_service::get_subscribe_details(state.repos.subscribe.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
