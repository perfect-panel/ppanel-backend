use axum::extract::State;

use crate::handler::AppState;
use crate::model::dto::common::GetStatResponse;
use crate::service::common::get_stat_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_stat(State(state): State<AppState>) -> HttpResult {
    match get_stat_service::get_stat(&state.repos).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<GetStatResponse>(None, Some(err)),
    }
}
