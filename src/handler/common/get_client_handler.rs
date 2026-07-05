use axum::extract::State;

use crate::handler::AppState;
use crate::model::dto::subscribe::GetSubscribeClientResponse;
use crate::service::common::get_client_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_client(State(state): State<AppState>) -> HttpResult {
    match get_client_service::get_client(&state.repos).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<GetSubscribeClientResponse>(None, Some(err)),
    }
}
