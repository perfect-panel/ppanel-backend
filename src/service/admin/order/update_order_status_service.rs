use crate::model::dto::order::UpdateOrderStatusRequest;
use crate::repository::order::OrderRepo;
use result::code_error::CodeError;
use result::error_code;

/// Update an order's lifecycle status (admin override).
///
/// The repo accepts a `&str` order_no; we resolve the `id` from the request
/// to the order_no via a lookup so the call signature matches what the rest
/// of the order services expect.  When the caller already supplies
/// `trade_no`/`payment_id` they are surfaced via tracing — persisting them
/// is intentionally left to the dedicated payment-completion flow so the
/// bookkeeping stays consistent.
pub async fn update_order_status(
    repo: &dyn OrderRepo,
    req: UpdateOrderStatusRequest,
) -> Result<(), anyhow::Error> {
    // Look up the order so we have its `order_no` for the repo call.
    let details = repo.find_one_details(req.id).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &format!("order {} not found: {}", req.id, e),
        ))
    })?;

    repo.update_order_status(&details.order_no, req.status as i16)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                &e.to_string(),
            ))
        })?;

    if req.trade_no.is_some() || req.payment_id.is_some() {
        tracing::info!(
            target: "service.admin.order",
            order_id = req.id,
            order_no = %details.order_no,
            payment_id = ?req.payment_id,
            trade_no = ?req.trade_no,
            "order status updated with payment context"
        );
    }

    Ok(())
}
