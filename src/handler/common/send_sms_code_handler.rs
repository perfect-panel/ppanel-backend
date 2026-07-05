use axum::{extract::State, Json};

use crate::handler::AppState;
use crate::model::dto::auth::{SendCodeResponse, SendSmsCodeRequest};
use crate::service::common::send_sms_code_service::SendSmsCodeService;
use result::http_result::{build_http_result, HttpResult};

pub async fn send_sms_code(
    State(state): State<AppState>,
    Json(req): Json<SendSmsCodeRequest>,
) -> HttpResult {
    let svc = SendSmsCodeService::new(state.repos.clone(), state.config.clone(), state.cache.clone(), state.queue.clone());
    match svc.send_code(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<SendCodeResponse>(None, Some(err)),
    }
}
