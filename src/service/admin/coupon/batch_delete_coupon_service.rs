use crate::model::dto::BatchDeleteCouponRequest;
use crate::repository::coupon::CouponRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn batch_delete_coupon(
    repo: &dyn CouponRepo,
    req: BatchDeleteCouponRequest,
) -> Result<(), anyhow::Error> {
    repo.batch_delete(&req.ids)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_DELETED_ERROR,
                &e.to_string(),
            ))
        })?;
    Ok(())
}
