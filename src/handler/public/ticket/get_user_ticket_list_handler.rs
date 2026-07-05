use axum::extract::{Query, State};
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::ticket::get_user_ticket_list_service::GetUserTicketListService;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_user_ticket_list(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Query(req): Query<GetUserTicketListRequest>,
) -> HttpResult {
    let svc = GetUserTicketListService::new(state.repos.clone());
    match svc.list(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
