//! Order-domain constants — ported from
//! `server/internal/logic/public/order/constant.go`.
//!
//! Only numeric limits and method-name strings that the order service
//! references are mirrored here. Order type / status values are
//! centralised in `queue::service::order` (matching Go's `service/order.go`).

/// Payment method names — match Go `constant.go` literals used as the
/// `method` column on `order`.
pub const EPAY: &str = "epay";
pub const ALIPAY_F2F: &str = "alipay_f2f";
pub const STRIPE_ALIPAY: &str = "stripe_alipay";
pub const STRIPE_WECHAT_PAY: &str = "stripe_wechat_pay";
pub const BALANCE: &str = "balance";

/// Order amount limits — ported verbatim from Go.
///
/// - `MAX_ORDER_AMOUNT` matches Go's `MaxOrderAmount = 2_147_483_647` (i32 max).
/// - `MAX_RECHARGE_AMOUNT` matches Go's `MaxRechargeAmount = 2_000_000_000`
///   (slightly lower for safety).
/// - `MAX_QUANTITY` matches Go's `MaxQuantity = 1000`.
pub const MAX_ORDER_AMOUNT: i64 = 2_147_483_647;
pub const MAX_RECHARGE_AMOUNT: i64 = 2_000_000_000;
pub const MAX_QUANTITY: i64 = 1_000;

/// Time before an unpaid order is auto-closed — matches Go's
/// `CloseOrderTimeMinutes = 15` in `purchaseLogic.go`.
pub const CLOSE_ORDER_TIME_MINUTES: i64 = 15;

/// Order type values — mirrored from Go's `order.Type` enum
/// (`1=new, 2=renewal, 3=reset_traffic, 4=recharge`).
///
/// Kept as `i16` to match the entity column type
/// (`Order.type_` is `TinyUint = i16`).
pub const ORDER_TYPE_SUBSCRIBE: i16 = 1;
pub const ORDER_TYPE_RENEWAL: i16 = 2;
pub const ORDER_TYPE_RESET_TRAFFIC: i16 = 3;
pub const ORDER_TYPE_RECHARGE: i16 = 4;

/// Order status values — mirrored from Go's `order.Status` enum
/// (`1=pending, 2=paid, 3=cancelled`). Stored as `i16` (`TinyUint`).
pub const ORDER_STATUS_UNPAID: i16 = 1;
pub const ORDER_STATUS_PAID: i16 = 2;
pub const ORDER_STATUS_CANCELLED: i16 = 3;

/// Coupon discount type — `1=percentage`, anything else = fixed amount.
/// Mirrors Go's `couponInfo.Type` discriminator in `calculateCoupon`.
pub const COUPON_TYPE_PERCENTAGE: i16 = 1;

/// Payment fee modes — mirror Go's `payment.FeeMode`.
/// `0` = no fee, `1` = percent, `2` = fixed, `3` = percent + fixed.
pub const FEE_MODE_NONE: i64 = 0;
pub const FEE_MODE_PERCENT: i64 = 1;
pub const FEE_MODE_FIXED: i64 = 2;
pub const FEE_MODE_PERCENT_PLUS_FIXED: i64 = 3;
