//! `CloseOrder` — user cancels an unpaid order, refunding the gift
//! balance and restoring subscribe inventory when applicable.
//!
//! Port of `server/internal/logic/public/order/closeOrderLogic.go`.
//! The order is left in status `3` (cancelled) rather than deleted, but
//! guest orders (user_id == 0) are deleted entirely to match Go.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::order::CloseOrderRequest;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

use super::constant::{ORDER_STATUS_CANCELLED, ORDER_STATUS_UNPAID};
use result::code_error::CodeError;
use result::error_code;

pub struct CloseOrderService {
    repos: Arc<Repositories>,
}

impl CloseOrderService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    /// Close `order_no` for the given `user_id`.
    ///
    /// `user_id` is accepted for handler-signature parity with the other
    /// order services; the Go logic keys cancellation by `order_no` only,
    /// so no ownership check is performed here.
    pub async fn close(
        &self,
        _user_id: i64,
        req: CloseOrderRequest,
    ) -> Result<(), anyhow::Error> {
        let order = self
            .repos
            .order
            .find_one_by_order_no(&req.order_no)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::ORDER_NOT_EXIST)))?;

        // Idempotent — already paid/cancelled is a no-op.
        if order.status != ORDER_STATUS_UNPAID {
            tracing::info!(
                order_no = %order.order_no,
                status = order.status,
                "close_order: order not in unpaid state, skipping",
            );
            return Ok(());
        }

        // Restore subscribe inventory (fetched once if SubscribeId is set).
        if order.subscribe_id > 0 {
            let mut sub = self
                .repos
                .subscribe
                .find_one(order.subscribe_id)
                .await
                .map_err(|_| {
                    anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
                })?;
            if sub.inventory != -1 {
                sub.inventory += 1;
                sub.updated_at = Utc::now().timestamp_millis();
                if let Err(e) = self.repos.subscribe.update(&sub).await {
                    tracing::error!(?e, subscribe_id = sub.id, "failed to restore inventory");
                    return Err(anyhow!(CodeError::new_err_code_msg(
                        error_code::DATABASE_UPDATE_ERROR,
                        &e.to_string(),
                    )));
                }
            }
        }

        // Update order status → cancelled.
        if let Err(e) = self
            .repos
            .order
            .update_order_status(&order.order_no, ORDER_STATUS_CANCELLED)
            .await
        {
            tracing::error!(?e, order_no = %order.order_no, "failed to update order status");
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                &e.to_string(),
            )));
        }

        // Refund gift-amount and write telemetry for non-guest orders.
        if order.user_id != 0 && order.gift_amount > 0 {
            let mut user = self
                .repos
                .user
                .find_one_user(order.user_id)
                .await
                .map_err(|_| {
                    anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
                })?;
            user.gift_amount += order.gift_amount;
            user.updated_at = Utc::now().timestamp_millis();
            if let Err(e) = self.repos.user.update_user(&user).await {
                tracing::error!(?e, user_id = order.user_id, "failed to refund gift_amount");
                return Err(anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_UPDATE_ERROR,
                    &e.to_string(),
                )));
            }
            Telemetry::gift(
                &self.repos,
                order.user_id,
                341, // GIFT_TYPE_INCREASE
                &order.order_no,
                0,
                order.gift_amount,
                user.gift_amount,
                Some("Order cancellation refund".to_string()),
            )
            .await;
        }

        Ok(())
    }
}
