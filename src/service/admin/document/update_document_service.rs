use chrono::Utc;

use crate::model::dto::UpdateDocumentRequest;
use crate::model::entity::document::Document as DocumentEntity;
use crate::repository::document::DocumentRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_document(
    repo: &dyn DocumentRepo,
    req: UpdateDocumentRequest,
) -> Result<(), anyhow::Error> {
    let mut entity: DocumentEntity = repo
        .find_one(req.id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    entity.title = req.title;
    entity.content = req.content;
    if let Some(tags) = req.tags {
        entity.tags = tags.join(",");
    }
    if let Some(v) = req.show {
        entity.show = Some(v);
    }
    entity.updated_at = Utc::now().timestamp_millis();

    repo.update(&entity)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                &e.to_string(),
            ))
        })?;

    Ok(())
}
