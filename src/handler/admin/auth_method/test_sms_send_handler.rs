use axum::extract::State;
use axum::Json;
use crate::handler::AppState;
use crate::service::admin::auth_method::get_auth_method_list_service;
use result::http_result::{build_http_result, HttpResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TestSmsSendRequest {
    pub to: String,
}

pub async fn test_sms_send(
    State(state): State<AppState>,
    Json(req): Json<TestSmsSendRequest>,
) -> HttpResult {
    match get_auth_method_list_service::test_sms_send(&state.config, req.to).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
