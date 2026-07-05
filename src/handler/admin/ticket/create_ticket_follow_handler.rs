use axum::extract::State;
use axum::Extension;
use axum::Json;
use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::ticket::CreateTicketFollowRequest;
use crate::service::admin::ticket::get_ticket_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_ticket_follow(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<CreateTicketFollowRequest>,
) -> HttpResult {
    match get_ticket_list_service::create_ticket_follow(
        state.repos.ticket.as_ref(),
        auth.user_id,
        req,
    ).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
