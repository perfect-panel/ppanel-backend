use axum::extract::State;

use crate::handler::AppState;
use crate::service::admin::console::query_user_statistics_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_user_statistics(State(state): State<AppState>) -> HttpResult {
    match query_user_statistics_service::query_user_statistics(
        state.repos.user.as_ref(),
        state.repos.order.as_ref(),
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
