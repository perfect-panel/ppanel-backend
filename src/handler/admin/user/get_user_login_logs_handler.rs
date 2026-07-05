use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::get_user_login_logs_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_user_login_logs(
    State(state): State<AppState>,
    Query(req): Query<GetUserLoginLogsRequest>,
) -> HttpResult {
    match get_user_login_logs_service::get_user_login_logs(&state.repos, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
