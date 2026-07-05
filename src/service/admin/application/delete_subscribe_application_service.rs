use crate::repository::client::ClientRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn delete_subscribe_application(
    repo: &dyn ClientRepo,
    id: i64,
) -> Result<(), anyhow::Error> {
    let affected = repo
        .delete(id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_DELETED_ERROR,
            &e.to_string(),
        )))?;

    if affected == 0 {
        return Err(anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_DELETED_ERROR,
            "delete subscribe application error: record not found",
        )));
    }

    Ok(())
}
