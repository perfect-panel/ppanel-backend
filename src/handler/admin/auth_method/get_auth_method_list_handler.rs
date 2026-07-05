use axum::extract::State;
use crate::handler::AppState;
use crate::service::admin::auth_method::get_auth_method_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_auth_method_list(
    State(state): State<AppState>,
) -> HttpResult {
    match get_auth_method_list_service::get_auth_method_list(state.repos.auth.as_ref()).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
