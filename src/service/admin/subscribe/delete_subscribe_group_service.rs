use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn delete_subscribe_group(
    repo: &dyn SubscribeRepo,
    id: i64,
) -> Result<(), anyhow::Error> {
    let affected = repo
        .delete_group(id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_DELETED_ERROR,
            &e.to_string(),
        )))?;

    if affected == 0 {
        return Err(anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_DELETED_ERROR,
            "delete subscribe group error: record not found",
        )));
    }

    Ok(())
}
