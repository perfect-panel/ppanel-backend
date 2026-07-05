use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::get_user_subscribe_logs_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_user_subscribe_logs(
    State(state): State<AppState>,
    Query(req): Query<GetUserSubscribeLogsRequest>,
) -> HttpResult {
    match get_user_subscribe_logs_service::get_user_subscribe_logs(
        &state.repos,
        req.user_id,
        req.page,
        req.size,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
