use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::document::get_document_detail_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_document_detail(
    State(state): State<AppState>,
    Query(req): Query<GetDocumentDetailRequest>,
) -> HttpResult {
    match get_document_detail_service::get_document_detail(state.repos.document.as_ref(), req.id)
        .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
