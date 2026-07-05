use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::ticket::update_user_ticket_status_service::UpdateUserTicketStatusService;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_user_ticket_status(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<UpdateUserTicketStatusRequest>,
) -> HttpResult {
    let svc = UpdateUserTicketStatusService::new(state.repos.clone());
    match svc.update(auth.user_id, req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
