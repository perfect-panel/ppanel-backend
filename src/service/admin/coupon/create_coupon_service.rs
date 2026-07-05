use chrono::Utc;

use crate::model::dto::{Coupon, CreateCouponRequest};
use crate::model::entity::coupon::Coupon as CouponEntity;
use crate::repository::coupon::CouponRepo;
use result::code_error::CodeError;
use result::error_code;

/// Coupon type constants (mirrors Go model)
const COUPON_TYPE_PERCENTAGE: u8 = 1;

/// Compute the stored `discount` value based on coupon type.
/// - percentage (1): discount field stores the percentage value directly (e.g. 10 = 10%)
/// - fixed (other): discount field stores the fixed amount
fn compute_discount(type_: u8, value: i64) -> i64 {
    // The discount field stores the raw value in both cases;
    // actual reduction calculation happens at order time.
    // Store as-is, consistent with Go behaviour (DeepCopy keeps discount as provided).
    let _ = COUPON_TYPE_PERCENTAGE;
    value
}

pub async fn create_coupon(
    repo: &dyn CouponRepo,
    req: CreateCouponRequest,
) -> Result<Coupon, anyhow::Error> {
    let now = Utc::now().timestamp_millis();

    // Auto-generate code if not provided
    let code = match req.code {
        Some(c) if !c.is_empty() => c,
        _ => {
            // Simple random code: timestamp-based hex
            format!("{:X}", now & 0xFFFFFFFF)
        }
    };

    let subscribe_str = req
        .subscribe
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let discount = compute_discount(req.type_, req.discount);

    let entity = CouponEntity {
        id: 0,
        name: req.name,
        code,
        count: req.count.unwrap_or(0),
        type_: req.type_ as i16,
        discount,
        start_time: req.start_time,
        expire_time: req.expire_time,
        user_limit: req.user_limit.unwrap_or(0),
        subscribe: subscribe_str,
        used_count: req.used_count.unwrap_or(0),
        enable: Some(req.enable.unwrap_or(true)),
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

    let subscribe_vec: Vec<i64> = if result.subscribe.is_empty() {
        vec![]
    } else {
        result
            .subscribe
            .split(',')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect()
    };

    Ok(Coupon {
        id: result.id,
        name: result.name,
        code: result.code,
        count: result.count,
        type_: result.type_ as u8,
        discount: result.discount,
        start_time: result.start_time,
        expire_time: result.expire_time,
        user_limit: result.user_limit,
        subscribe: subscribe_vec,
        used_count: result.used_count,
        enable: result.enable.unwrap_or(false),
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}
