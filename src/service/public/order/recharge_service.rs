//! `Recharge` — create a balance top-up order.
//!
//! Port of `server/internal/logic/public/order/rechargeLogic.go`.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::order::{RechargeOrderRequest, RechargeOrderResponse};
use crate::model::entity::order::Order;
use crate::queue::client::QueueClient;
use crate::queue::types::DEFER_CLOSE_ORDER;
use crate::repository::Repositories;

use super::calculate_fee::calculate_fee;
use super::constant::{MAX_ORDER_AMOUNT, MAX_RECHARGE_AMOUNT, ORDER_STATUS_UNPAID, ORDER_TYPE_RECHARGE};
use result::code_error::CodeError;
use result::error_code;

pub struct RechargeService {
    repos: Arc<Repositories>,
    queue: QueueClient,
}

impl RechargeService {
    pub fn new(repos: Arc<Repositories>, queue: QueueClient) -> Self {
        Self { repos, queue }
    }

    pub async fn recharge(
        &self,
        user_id: i64,
        req: RechargeOrderRequest,
    ) -> Result<RechargeOrderResponse, anyhow::Error> {
        if req.amount <= 0 {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "recharge amount must be greater than 0",
            )));
        }
        if req.amount > MAX_RECHARGE_AMOUNT {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "recharge amount exceeds maximum limit",
            )));
        }

        let payment_id = req.payment.ok_or_else(|| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "payment method is required",
            ))
        })?;

        let payment = self
            .repos
            .payment
            .find_one(payment_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let fee_amount = calculate_fee(req.amount, &payment);
        let total_amount = req.amount + fee_amount;
        if total_amount > MAX_ORDER_AMOUNT {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "total amount exceeds maximum limit",
            )));
        }

        let is_new = self
            .repos
            .order
            .is_user_eligible_for_new_order(user_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let now = Utc::now().timestamp_millis();
        let order_no = super::purchase_service::generate_trade_no();
        let order = Order {
            id: 0,
            parent_id: None,
            user_id,
            order_no: order_no.clone(),
            type_: ORDER_TYPE_RECHARGE,
            quantity: 0,
            price: req.amount,
            amount: total_amount,
            gift_amount: 0,
            discount: 0,
            coupon: None,
            coupon_discount: 0,
            commission: 0,
            payment_id: payment.id,
            method: payment.platform.clone(),
            fee_amount,
            trade_no: None,
            status: ORDER_STATUS_UNPAID,
            subscribe_id: 0,
            subscribe_token: None,
            is_new,
            created_at: now,
            updated_at: now,
        };

        if let Err(e) = self.repos.order.insert(&order).await {
            tracing::error!(?e, %order_no, "failed to insert recharge order");
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            )));
        }

        self.enqueue_close_task(&order_no).await;

        Ok(RechargeOrderResponse { order_no })
    }

    async fn enqueue_close_task(&self, order_no: &str) {
        let delay = std::time::Duration::from_secs(
            (super::constant::CLOSE_ORDER_TIME_MINUTES as u64) * 60,
        );
        let payload = match serde_json::to_vec(order_no) {
            Ok(b) => b,
            Err(e) => {
                tracing::error!(order_no, "failed to serialize close-order payload: {e}");
                return;
            }
        };
        if let Err(e) = self.queue.enqueue_delayed(DEFER_CLOSE_ORDER, &payload, delay).await {
            tracing::error!(order_no, "failed to enqueue defer close-order task: {e}");
        }
    }
}
