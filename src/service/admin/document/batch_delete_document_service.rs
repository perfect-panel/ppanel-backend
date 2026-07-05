use crate::model::dto::BatchDeleteDocumentRequest;
use crate::repository::document::DocumentRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn batch_delete_document(
    repo: &dyn DocumentRepo,
    req: BatchDeleteDocumentRequest,
) -> Result<(), anyhow::Error> {
    // Use the repo's delete method for each id; there's no batch_delete on DocumentRepo,
    // so delete individually.
    for id in req.ids {
        repo.delete(id)
            .await
            .map_err(|e| {
                anyhow::Error::new(CodeError::new_err_code_msg(
                    error_code::DATABASE_DELETED_ERROR,
                    &e.to_string(),
                ))
            })?;
    }
    Ok(())
}
