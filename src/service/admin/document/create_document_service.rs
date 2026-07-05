use chrono::Utc;

use crate::model::dto::{CreateDocumentRequest, Document};
use crate::model::entity::document::Document as DocumentEntity;
use crate::repository::document::DocumentRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_document(
    repo: &dyn DocumentRepo,
    req: CreateDocumentRequest,
) -> Result<Document, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let tags_str = req
        .tags
        .as_deref()
        .unwrap_or(&[])
        .join(",");
    let entity = DocumentEntity {
        id: 0,
        title: req.title,
        content: req.content,
        tags: tags_str,
        show: req.show,
        created_at: now,
        updated_at: now,
    };
    let result = repo
        .insert(&entity)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            ))
        })?;

    let tags_vec: Vec<String> = if result.tags.is_empty() {
        vec![]
    } else {
        result
            .tags
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    };

    Ok(Document {
        id: result.id,
        title: result.title,
        content: result.content,
        tags: tags_vec,
        show: result.show.unwrap_or(false),
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}
