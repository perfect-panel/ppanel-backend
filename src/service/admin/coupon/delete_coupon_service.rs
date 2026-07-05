use crate::repository::coupon::CouponRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn delete_coupon(
    repo: &dyn CouponRepo,
    id: i64,
) -> Result<(), anyhow::Error> {
    let affected = repo
        .delete(id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_DELETED_ERROR,
                &e.to_string(),
            ))
        })?;

    if affected == 0 {
        return Err(anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_DELETED_ERROR,
            "delete coupon error: record not found",
        )));
    }

    Ok(())
}
