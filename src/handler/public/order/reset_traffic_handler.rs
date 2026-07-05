use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::order::reset_traffic_service::ResetTrafficService;
use result::http_result::{build_http_result, HttpResult};

pub async fn reset_traffic(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<ResetTrafficOrderRequest>,
) -> HttpResult {
    let svc = ResetTrafficService::new(state.repos.clone(), state.queue.clone());
    match svc.reset_traffic(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
