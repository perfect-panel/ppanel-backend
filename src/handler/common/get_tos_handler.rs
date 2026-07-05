use axum::extract::State;

use crate::handler::AppState;
use crate::model::dto::common::GetTosResponse;
use crate::service::common::get_tos_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_tos(State(state): State<AppState>) -> HttpResult {
    match get_tos_service::get_tos(&state.repos).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<GetTosResponse>(None, Some(err)),
    }
}
