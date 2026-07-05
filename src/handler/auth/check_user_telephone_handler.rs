use axum::{extract::State, Json};

use crate::handler::AppState;
use crate::model::dto::auth::{TelephoneCheckUserRequest, TelephoneCheckUserResponse};
use crate::service::auth::check_user_telephone_service::CheckUserTelephoneService;
use result::http_result::{build_http_result, HttpResult};

pub async fn check_user_telephone(
    State(state): State<AppState>,
    Json(req): Json<TelephoneCheckUserRequest>,
) -> HttpResult {
    let svc = CheckUserTelephoneService::new(state.repos.clone(), state.config.clone());
    match svc.check(req).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<TelephoneCheckUserResponse>(None, Some(err)),
    }
}
