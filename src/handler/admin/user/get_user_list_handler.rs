use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::get_user_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_user_list(
    State(state): State<AppState>,
    Query(req): Query<GetUserListRequest>,
) -> HttpResult {
    match get_user_list_service::get_user_list(&state.repos, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
