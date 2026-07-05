//! Resolve the quantity-based discount rate for a subscribe plan.
//!
//! Direct port of `server/internal/logic/public/order/getDiscount.go`.
//!
//! Go's `Subscribe.Discount` is a JSON column of `[]types.SubscribeDiscount`,
//! where each entry has `{ quantity int64, discount int64 }`. The function
//! picks the smallest `discount` whose `quantity <= input_months` and
//! returns it as a fraction (i.e. `discount/100`).

use serde::{Deserialize, Serialize};

/// Single discount tier deserialised from `Subscribe.discount` JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeDiscount {
    /// Threshold: tiers with `quantity > input_months` are skipped.
    pub quantity: i64,
    /// Discount value, where `100` = no discount, `80` = 20% off, etc.
    /// Stored as a percentage (`0..=100`) for parity with Go.
    pub discount: i64,
}

/// Parse `Subscribe.discount` JSON. Returns an empty vec on parse failure
/// — matches Go's `_ = json.Unmarshal([]byte(sub.Discount), &dis)` and the
/// `if sub.Discount != ""` guard.
pub fn parse_discounts(json: &str) -> Vec<SubscribeDiscount> {
    if json.is_empty() {
        return Vec::new();
    }
    serde_json::from_str(json).unwrap_or_default()
}

/// Resolve the discount multiplier for `input_months`.
///
/// - Returns `1.0` (no discount) when no tier qualifies.
/// - Returns `min_discount / 100.0` otherwise.
///
/// Mirrors Go's loop:
/// ```text
/// var finalDiscount float64 = 100
/// for _, discount := range discounts {
///     if inputMonths >= discount.Quantity && discount.Discount < finalDiscount {
///         finalDiscount = discount.Discount
///     }
/// }
/// return finalDiscount / float64(100)
/// ```
pub fn get_discount(discounts: &[SubscribeDiscount], input_months: i64) -> f64 {
    let mut final_discount: i64 = 100;
    for d in discounts {
        if input_months >= d.quantity && d.discount < final_discount {
            final_discount = d.discount;
        }
    }
    final_discount as f64 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_discounts_returns_one() {
        assert_eq!(get_discount(&[], 5), 1.0);
    }

    #[test]
    fn below_threshold_returns_one() {
        let discounts = vec![SubscribeDiscount {
            quantity: 3,
            discount: 80,
        }];
        assert_eq!(get_discount(&discounts, 2), 1.0);
    }

    #[test]
    fn picks_smallest_qualifying_discount() {
        let discounts = vec![
            SubscribeDiscount {
                quantity: 1,
                discount: 95,
            },
            SubscribeDiscount {
                quantity: 3,
                discount: 80,
            },
            SubscribeDiscount {
                quantity: 6,
                discount: 70,
            },
        ];
        // 5 months → 80% is the lowest qualifying (q<=5)
        assert_eq!(get_discount(&discounts, 5), 0.80);
        // 12 months → 70%
        assert_eq!(get_discount(&discounts, 12), 0.70);
    }

    #[test]
    fn parse_discounts_returns_empty_on_garbage() {
        assert!(parse_discounts("not json").is_empty());
    }

    #[test]
    fn parse_discounts_roundtrip() {
        let json = r#"[{"quantity":3,"discount":80},{"quantity":6,"discount":70}]"#;
        let v = parse_discounts(json);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].discount, 80);
    }
}
