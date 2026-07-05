use chrono::Utc;

use crate::model::dto::UpdateCouponRequest;
use crate::model::entity::coupon::Coupon as CouponEntity;
use crate::repository::coupon::CouponRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_coupon(
    repo: &dyn CouponRepo,
    req: UpdateCouponRequest,
) -> Result<(), anyhow::Error> {
    let mut entity: CouponEntity = repo
        .find_one(req.id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    entity.name = req.name;
    if let Some(code) = req.code {
        if !code.is_empty() {
            entity.code = code;
        }
    }
    if let Some(count) = req.count {
        entity.count = count;
    }
    entity.type_ = req.type_ as i16;
    entity.discount = req.discount;
    entity.start_time = req.start_time;
    entity.expire_time = req.expire_time;
    if let Some(user_limit) = req.user_limit {
        entity.user_limit = user_limit;
    }
    if let Some(subscribe) = req.subscribe {
        entity.subscribe = subscribe
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
    }
    if let Some(used_count) = req.used_count {
        entity.used_count = used_count;
    }
    if let Some(enable) = req.enable {
        entity.enable = Some(enable);
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
