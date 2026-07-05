use axum::extract::State;
use axum::Json;
use crate::handler::AppState;
use crate::model::dto::ticket::UpdateTicketStatusRequest;
use crate::service::admin::ticket::get_ticket_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_ticket_status(
    State(state): State<AppState>,
    Json(req): Json<UpdateTicketStatusRequest>,
) -> HttpResult {
    match get_ticket_list_service::update_ticket_status(state.repos.ticket.as_ref(), req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
