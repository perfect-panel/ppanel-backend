//! Portal-scoped utilities.

use uuid::Uuid;

/// Generate a short unique trade/order number.
///
/// Mirrors Go's `tool.GenerateTradeNo` — 16 uppercase hex chars.
pub fn generate_trade_no() -> String {
    let s = Uuid::new_v4().simple().to_string();
    s[..16].to_uppercase()
}
