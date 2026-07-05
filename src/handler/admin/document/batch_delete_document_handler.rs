use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::document::batch_delete_document_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn batch_delete_document(
    State(state): State<AppState>,
    Json(req): Json<BatchDeleteDocumentRequest>,
) -> HttpResult {
    match batch_delete_document_service::batch_delete_document(state.repos.document.as_ref(), req)
        .await
    {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
