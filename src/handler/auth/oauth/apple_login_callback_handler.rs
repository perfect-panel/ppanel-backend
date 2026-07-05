use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Json,
};

use crate::handler::AppState;
use crate::model::dto::auth::AppleLoginCallbackRequest;
use crate::service::auth::oauth::apple_login_callback_service::AppleLoginCallbackService;
use result::http_result::{build_http_result, HttpResult};

pub async fn apple_login_callback(
    State(state): State<AppState>,
    Json(req): Json<AppleLoginCallbackRequest>,
) -> Response {
    let svc = AppleLoginCallbackService::new(
        state.repos.clone(), state.config.clone(), state.cache.clone(),
    );
    match svc.callback(req).await {
        Ok(url) => Redirect::temporary(&url).into_response(),
        Err(err) => {
            let result: HttpResult = build_http_result::<()>(None, Some(err));
            result.into_response()
        }
    }
}
