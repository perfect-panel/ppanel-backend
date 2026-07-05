use axum::extract::State;

use crate::handler::AppState;
use crate::model::dto::common::GetGlobalConfigResponse;
use crate::service::common::get_global_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_global_config(State(state): State<AppState>) -> HttpResult {
    match get_global_config_service::get_global_config(&state.repos, &state.config).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<GetGlobalConfigResponse>(None, Some(err)),
    }
}
