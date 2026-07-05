use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::subscribe::get_subscribe_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_subscribe_list(
    State(state): State<AppState>,
    Query(req): Query<GetSubscribeListRequest>,
) -> HttpResult {
    match get_subscribe_list_service::get_subscribe_list(state.repos.subscribe.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
