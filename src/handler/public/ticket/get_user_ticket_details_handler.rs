use axum::extract::{Query, State};
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::ticket::get_user_ticket_details_service::GetUserTicketDetailsService;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_user_ticket_details(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Query(req): Query<GetUserTicketDetailRequest>,
) -> HttpResult {
    let svc = GetUserTicketDetailsService::new(state.repos.clone());
    match svc.get(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
