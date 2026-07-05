use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::document::get_document_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_document_list(
    State(state): State<AppState>,
    Query(req): Query<GetDocumentListRequest>,
) -> HttpResult {
    match get_document_list_service::get_document_list(state.repos.document.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
