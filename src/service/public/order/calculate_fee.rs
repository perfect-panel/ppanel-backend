//! Calculate the payment handling fee for a given amount + payment config.
//!
//! Direct port of `server/internal/logic/public/order/calculateFee.go`. The
//! fee logic in Go depends only on `payment.FeeMode`, `FeePercent` and
//! `FeeAmount`, so we mirror the same fields on the [`Payment`] entity.

use crate::model::entity::payment::Payment;

use super::constant::{
    FEE_MODE_FIXED, FEE_MODE_NONE, FEE_MODE_PERCENT, FEE_MODE_PERCENT_PLUS_FIXED,
};

/// Compute the handling fee for `amount` using `payment`'s fee mode.
///
/// Mode semantics (matches `payment.FeeMode` in Go):
/// - `FEE_MODE_NONE` (`0`)                → always 0.
/// - `FEE_MODE_PERCENT` (`1`)             → `amount * FeePercent / 100`.
/// - `FEE_MODE_FIXED` (`2`)               → `FeeAmount` when `amount > 0`, else 0.
/// - `FEE_MODE_PERCENT_PLUS_FIXED` (`3`)  → `(amount * FeePercent / 100) + FeeAmount`.
pub fn calculate_fee(amount: i64, payment: &Payment) -> i64 {
    let fee_percent = payment.fee_percent;
    let fee_amount = payment.fee_amount;

    match payment.fee_mode {
        FEE_MODE_NONE => 0,
        FEE_MODE_PERCENT => amount.saturating_mul(fee_percent) / 100,
        FEE_MODE_FIXED => {
            if amount > 0 {
                fee_amount
            } else {
                0
            }
        }
        FEE_MODE_PERCENT_PLUS_FIXED => {
            amount.saturating_mul(fee_percent) / 100 + fee_amount
        }
        // Unknown mode — Go's switch silently falls through and returns 0.
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payment(mode: i64, percent: i64, fixed: i64) -> Payment {
        Payment {
            id: 0,
            name: String::new(),
            platform: String::new(),
            icon: String::new(),
            domain: String::new(),
            config: String::new(),
            description: None,
            fee_mode: mode,
            fee_percent: percent,
            fee_amount: fixed,
            sort: 0,
            enable: None,
            token: String::new(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn none_mode_returns_zero() {
        assert_eq!(calculate_fee(100, &payment(FEE_MODE_NONE, 10, 5)), 0);
    }

    #[test]
    fn percent_mode_uses_amount() {
        // 200 * 10 / 100 = 20
        assert_eq!(calculate_fee(200, &payment(FEE_MODE_PERCENT, 10, 999)), 20);
    }

    #[test]
    fn fixed_mode_zero_amount_returns_zero() {
        assert_eq!(calculate_fee(0, &payment(FEE_MODE_FIXED, 0, 9)), 0);
    }

    #[test]
    fn fixed_mode_positive_amount_returns_fixed() {
        assert_eq!(calculate_fee(1, &payment(FEE_MODE_FIXED, 0, 9)), 9);
    }

    #[test]
    fn percent_plus_fixed_mode_sums_both() {
        // 1000 * 2 / 100 + 5 = 25
        assert_eq!(
            calculate_fee(1000, &payment(FEE_MODE_PERCENT_PLUS_FIXED, 2, 5)),
            25
        );
    }
}
