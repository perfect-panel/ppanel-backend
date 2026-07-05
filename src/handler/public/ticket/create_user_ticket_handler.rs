use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::ticket::create_user_ticket_service::CreateUserTicketService;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_user_ticket(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<CreateUserTicketRequest>,
) -> HttpResult {
    let svc = CreateUserTicketService::new(state.repos.clone());
    match svc.create(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
