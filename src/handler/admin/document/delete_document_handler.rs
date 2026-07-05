use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::document::delete_document_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_document(
    State(state): State<AppState>,
    Json(req): Json<DeleteDocumentRequest>,
) -> HttpResult {
    match delete_document_service::delete_document(state.repos.document.as_ref(), req.id).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
