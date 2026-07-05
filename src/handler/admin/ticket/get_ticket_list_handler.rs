use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::model::dto::ticket::GetTicketListRequest;
use crate::service::admin::ticket::get_ticket_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_ticket_list(
    State(state): State<AppState>,
    Query(req): Query<GetTicketListRequest>,
) -> HttpResult {
    match get_ticket_list_service::get_ticket_list(state.repos.ticket.as_ref(), req).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
