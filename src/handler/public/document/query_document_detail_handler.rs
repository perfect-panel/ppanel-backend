use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::service::public::document::query_document_detail_service::QueryDocumentDetailService;
use result::http_result::{build_http_result, HttpResult};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryDocumentDetailRequest { pub id: Option<i64>, pub slug: Option<String> }

pub async fn query_document_detail(
    State(state): State<AppState>,
    Query(req): Query<QueryDocumentDetailRequest>,
) -> HttpResult {
    let svc = QueryDocumentDetailService::new(state.repos.clone());
    let id = req.id.unwrap_or(0);
    match svc.query_detail(id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
