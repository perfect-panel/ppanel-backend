use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::subscribe::get_subscribe_group_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_subscribe_group_list(
    State(state): State<AppState>,
) -> HttpResult {
    match get_subscribe_group_list_service::get_subscribe_group_list(state.repos.subscribe.as_ref()).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
