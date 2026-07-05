use crate::model::dto::{Document, GetDocumentListRequest, GetDocumentListResponse};
use crate::repository::document::DocumentRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_document_list(
    repo: &dyn DocumentRepo,
    req: GetDocumentListRequest,
) -> Result<GetDocumentListResponse, anyhow::Error> {
    let (total, items) = repo
        .query_list(
            req.page,
            req.size,
            req.tag.as_deref(),
            req.search.as_deref(),
        )
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let list: Vec<Document> = items
        .into_iter()
        .map(|e| {
            let tags_vec: Vec<String> = if e.tags.is_empty() {
                vec![]
            } else {
                e.tags
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect()
            };
            Document {
                id: e.id,
                title: e.title,
                content: e.content,
                tags: tags_vec,
                show: e.show.unwrap_or(false),
                created_at: e.created_at,
                updated_at: e.updated_at,
            }
        })
        .collect();

    Ok(GetDocumentListResponse { total, list })
}
