//! Coupon discount calculation + enabled-state guard.
//!
//! Direct port of `server/internal/logic/public/order/calculateCoupon.go`.

use anyhow::anyhow;

use crate::model::entity::coupon::Coupon;

use super::constant::COUPON_TYPE_PERCENTAGE;
use result::code_error::CodeError;
use result::error_code;

/// Coupon is unusable if `enable == Some(false)`. Mirrors Go's
/// `coupon.Coupon.IsEnabled()` (which returns true when `enable` is nil
/// or true, false only when explicitly disabled).
pub fn ensure_coupon_enabled(coupon_info: &Coupon) -> Result<(), anyhow::Error> {
    match coupon_info.enable {
        Some(false) => Err(anyhow!(CodeError::new_err_code(error_code::COUPON_DISABLED))),
        _ => Ok(()),
    }
}

/// Compute the discount amount for a given order amount and coupon.
///
/// - `COUPON_TYPE_PERCENTAGE` (`1`) → `amount * Coupon.Discount / 100`
///   (truncated to integer).
/// - Any other type → `min(Coupon.Discount, amount)`.
pub fn calculate_coupon(amount: i64, coupon_info: &Coupon) -> i64 {
    if coupon_info.type_ == COUPON_TYPE_PERCENTAGE {
        amount.saturating_mul(coupon_info.discount) / 100
    } else {
        // Fixed amount discount — never exceeds the order total.
        coupon_info.discount.min(amount)
    }
}

/// Re-export `ensure_coupon_enabled` so the order pre-create and purchase
/// services can call it without exposing the helper above.
pub use ensure_coupon_enabled as ensure_enabled;

#[cfg(test)]
mod tests {
    use super::*;

    fn coupon(type_: i16, discount: i64, enable: Option<bool>) -> Coupon {
        Coupon {
            id: 0,
            name: String::new(),
            code: String::new(),
            count: 0,
            type_,
            discount,
            start_time: 0,
            expire_time: 0,
            user_limit: 0,
            subscribe: String::new(),
            used_count: 0,
            enable,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn percentage_coupon_uses_amount() {
        // 1000 * 20 / 100 = 200
        assert_eq!(
            calculate_coupon(1000, &coupon(COUPON_TYPE_PERCENTAGE, 20, None)),
            200
        );
    }

    #[test]
    fn fixed_coupon_uses_discount_value() {
        assert_eq!(calculate_coupon(1000, &coupon(0, 50, None)), 50);
    }

    #[test]
    fn fixed_coupon_caps_at_amount() {
        // 1000 amount but 5000 fixed → cap at 1000
        assert_eq!(calculate_coupon(1000, &coupon(0, 5000, None)), 1000);
    }

    #[test]
    fn ensure_enabled_rejects_explicitly_disabled() {
        let c = coupon(0, 10, Some(false));
        assert!(ensure_coupon_enabled(&c).is_err());
    }

    #[test]
    fn ensure_enabled_accepts_enabled_or_nil() {
        let c1 = coupon(0, 10, Some(true));
        let c2 = coupon(0, 10, None);
        assert!(ensure_coupon_enabled(&c1).is_ok());
        assert!(ensure_coupon_enabled(&c2).is_ok());
    }
}
