use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::user::update_user_subscribe_service;
use result::code_error::CodeError;
use result::error_code;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_user_subscribe(
    State(state): State<AppState>,
    Json(req): Json<UpdateUserSubscribeRequest>,
) -> HttpResult {
    let mut sub = match state.repos.user.find_one_subscribe(req.user_subscribe_id).await {
        Ok(s) => s,
        Err(e) => {
            return build_http_result::<()>(
                None,
                Some(anyhow::Error::new(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    &e.to_string(),
                ))),
            );
        }
    };
    sub.subscribe_id = req.subscribe_id;
    sub.traffic = req.traffic;
    sub.expire_time = req.expired_at;
    sub.upload = req.upload;
    sub.download = req.download;
    match update_user_subscribe_service::update_user_subscribe(&state.repos, sub).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
