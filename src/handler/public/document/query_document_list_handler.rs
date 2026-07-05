use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::model::dto::document::GetDocumentListRequest;
use crate::service::public::document::query_document_list_service::QueryDocumentListService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_document_list(
    State(state): State<AppState>,
    Query(req): Query<GetDocumentListRequest>,
) -> HttpResult {
    let svc = QueryDocumentListService::new(state.repos.clone());
    match svc.query_list(req.page, req.size, req.tag.as_deref()).await {
        Ok((total, list)) => build_http_result(Some(serde_json::json!({"total": total, "list": list})), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
