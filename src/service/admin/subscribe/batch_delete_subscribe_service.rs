use crate::model::dto::subscribe::BatchDeleteSubscribeRequest;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn batch_delete_subscribe(
    repo: &dyn SubscribeRepo,
    req: BatchDeleteSubscribeRequest,
) -> Result<u64, anyhow::Error> {
    // TODO: replace with a dedicated batch delete repo method when added.
    // For now, delete one by one and sum the affected rows.
    let mut total: u64 = 0;
    for id in req.ids {
        let affected = repo
            .delete(id)
            .await
            .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_DELETED_ERROR,
                &e.to_string(),
            )))?;
        total = total.saturating_add(affected);
    }
    Ok(total)
}
