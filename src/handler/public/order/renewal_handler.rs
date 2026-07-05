use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::order::renewal_service::RenewalService;
use result::http_result::{build_http_result, HttpResult};

pub async fn renewal(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<RenewalOrderRequest>,
) -> HttpResult {
    let svc = RenewalService::new(state.repos.clone(), state.queue.clone());
    match svc.renewal(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
