use crate::model::dto::subscribe::BatchDeleteSubscribeGroupRequest;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn batch_delete_subscribe_group(
    repo: &dyn SubscribeRepo,
    req: BatchDeleteSubscribeGroupRequest,
) -> Result<u64, anyhow::Error> {
    repo.batch_delete_group(&req.ids)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_DELETED_ERROR,
            &e.to_string(),
        )))
}
