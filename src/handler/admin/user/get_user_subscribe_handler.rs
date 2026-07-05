use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use result::http_result::{build_http_result, HttpResult};
use result::code_error::CodeError;
use result::error_code;

pub async fn get_user_subscribe(
    State(state): State<AppState>,
    Query(req): Query<GetUserSubscribeListRequest>,
) -> HttpResult {
    let result = state.repos.user
        .query_user_subscribe(req.user_id, &[0, 1, 2, 3, 4, 5])
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, &e.to_string())));
    match result {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
