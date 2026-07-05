use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::document::update_document_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_document(
    State(state): State<AppState>,
    Json(req): Json<UpdateDocumentRequest>,
) -> HttpResult {
    match update_document_service::update_document(state.repos.document.as_ref(), req).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
