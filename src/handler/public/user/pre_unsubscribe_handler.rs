use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::pre_unsubscribe_service::PreUnsubscribeService;
use result::http_result::{build_http_result, HttpResult};

pub async fn pre_unsubscribe(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<PreUnsubscribeRequest>,
) -> HttpResult {
    let svc = PreUnsubscribeService::new(state.repos.clone());
    match svc.pre_unsubscribe(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
