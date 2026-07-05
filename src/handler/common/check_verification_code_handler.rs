use axum::{extract::State, Json};

use crate::handler::AppState;
use crate::model::dto::auth::{CheckVerificationCodeRequest, CheckVerificationCodeRespone};
use crate::service::common::check_verification_code_service::CheckVerificationCodeService;
use result::http_result::{build_http_result, HttpResult};

pub async fn check_verification_code(
    State(state): State<AppState>,
    Json(req): Json<CheckVerificationCodeRequest>,
) -> HttpResult {
    let svc = CheckVerificationCodeService::new(state.repos.clone(), state.config.clone(), state.cache.clone());
    match svc.check(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<CheckVerificationCodeRespone>(None, Some(err)),
    }
}
