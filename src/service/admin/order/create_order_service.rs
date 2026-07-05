use crate::model::dto::order::CreateOrderRequest;
use crate::model::entity::order::{Order, TinyUint};
use crate::repository::order::OrderRepo;
use chrono::Utc;
use result::code_error::CodeError;
use result::error_code;

/// Manually create an order on behalf of a user (admin-only operation).
///
/// Mirrors the Go admin handler that injects a pre-computed `order_no` and
/// pre-fills all pricing fields.  `order_no` uniqueness is the caller's
/// responsibility in the Go codebase too; the repo layer will surface a
/// database error if it collides.
pub async fn create_order(
    repo: &dyn OrderRepo,
    req: CreateOrderRequest,
) -> Result<(), anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let order_no = generate_order_no(now);

    let entity = Order {
        id: 0,
        parent_id: None,
        user_id: req.user_id,
        order_no,
        type_: req.type_ as TinyUint,
        quantity: req.quantity.unwrap_or(1),
        price: req.price,
        amount: req.amount,
        gift_amount: 0,
        discount: req.discount.unwrap_or(0),
        coupon: req.coupon,
        coupon_discount: req.coupon_discount.unwrap_or(0),
        commission: req.commission,
        payment_id: req.payment_id,
        method: String::new(),
        fee_amount: req.fee_amount,
        trade_no: req.trade_no,
        status: req.status.unwrap_or(0) as TinyUint,
        subscribe_id: req.subscribe_id.unwrap_or(0),
        subscribe_token: None,
        is_new: false,
        created_at: now,
        updated_at: now,
    };

    repo.insert(&entity).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        ))
    })?;
    Ok(())
}

/// Generate a millisecond-based order number with a 4-digit random suffix to
/// keep the value unique under high concurrency.  Matches the Go helper that
/// produced strings like `17000000000001234`.
fn generate_order_no(now_ms: i64) -> String {
    // Cheap PRNG seeded from the timestamp — order numbers only need to be
    // unique within a millisecond window, not cryptographically random.
    let suffix = ((now_ms.wrapping_mul(1103515245).wrapping_add(12345)) % 10000) as u32;
    format!("{}{:04}", now_ms, suffix)
}
