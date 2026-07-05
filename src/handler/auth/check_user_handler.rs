use axum::{extract::State, Json};

use crate::handler::AppState;
use crate::model::dto::auth::{CheckUserRequest, CheckUserResponse};
use crate::service::auth::check_user_service::CheckUserService;
use result::http_result::{build_http_result, HttpResult};

pub async fn check_user(
    State(state): State<AppState>,
    Json(req): Json<CheckUserRequest>,
) -> HttpResult {
    let svc = CheckUserService::new(state.repos.clone(), state.config.clone());
    match svc.check(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<CheckUserResponse>(None, Some(err)),
    }
}
