//! `ResetTraffic` — create a paid traffic-reset order against an
//! existing user-subscription.
//!
//! Port of `server/internal/logic/public/order/resetTrafficLogic.go`.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::order::{ResetTrafficOrderRequest, ResetTrafficOrderResponse};
use crate::model::entity::order::Order;
use crate::queue::client::QueueClient;
use crate::queue::types::DEFER_CLOSE_ORDER;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

use super::calculate_fee::calculate_fee;
use super::constant::{ORDER_STATUS_UNPAID, ORDER_TYPE_RESET_TRAFFIC};
use super::purchase_service::generate_trade_no;
use result::code_error::CodeError;
use result::error_code;

pub struct ResetTrafficService {
    repos: Arc<Repositories>,
    queue: QueueClient,
}

impl ResetTrafficService {
    pub fn new(repos: Arc<Repositories>, queue: QueueClient) -> Self {
        Self { repos, queue }
    }

    pub async fn reset_traffic(
        &self,
        user_id: i64,
        req: ResetTrafficOrderRequest,
    ) -> Result<ResetTrafficOrderResponse, anyhow::Error> {
        let mut user = self
            .repos
            .user
            .find_one_user(user_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)))?;

        let user_sub = self
            .repos
            .user
            .find_one_subscribe_details_by_id(req.user_subscribe_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        // Replacement amount — the value the user must pay to reset.
        // In the Go reference this is read from `userSubscribe.Subscribe.Replacement`,
        // which is the `replacement` column on the subscribe plan joined to the
        // user-subscribe row. The repository's `SubscribeDetails` doesn't carry
        // it; we fetch the plan here when needed.
        let sub = self
            .repos
            .subscribe
            .find_one(user_sub.subscribe_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;
        let replacement = sub.replacement;

        let mut amount = replacement;
        let mut deduction_amount: i64 = 0;
        if user.gift_amount > 0 {
            if user.gift_amount >= amount {
                deduction_amount = amount;
                user.gift_amount -= amount;
                amount = 0;
            } else {
                deduction_amount = user.gift_amount;
                amount -= user.gift_amount;
                user.gift_amount = 0;
            }
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

        let mut fee_amount: i64 = 0;
        if amount > 0 {
            fee_amount = calculate_fee(amount, &payment);
        }
        let final_amount = amount + fee_amount;

        let now = Utc::now().timestamp_millis();
        let order_no = generate_trade_no();
        let order = Order {
            id: 0,
            parent_id: Some(user_sub.order_id),
            user_id,
            order_no: order_no.clone(),
            type_: ORDER_TYPE_RESET_TRAFFIC,
            quantity: 0,
            price: replacement,
            amount: final_amount,
            gift_amount: deduction_amount,
            discount: 0,
            coupon: None,
            coupon_discount: 0,
            commission: 0,
            payment_id: payment.id,
            method: payment.platform.clone(),
            fee_amount,
            trade_no: None,
            status: ORDER_STATUS_UNPAID,
            subscribe_id: user_sub.subscribe_id,
            subscribe_token: Some(user_sub.token.clone()),
            is_new: false,
            created_at: now,
            updated_at: now,
        };

        if deduction_amount > 0 {
            user.updated_at = now;
            if let Err(e) = self.repos.user.update_user(&user).await {
                tracing::error!(?e, user_id, "failed to deduct gift_amount on traffic reset");
                return Err(anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_UPDATE_ERROR,
                    &e.to_string(),
                )));
            }
            Telemetry::gift(
                &self.repos,
                user_id,
                342, // GIFT_TYPE_REDUCE
                &order_no,
                0,
                deduction_amount,
                user.gift_amount,
                Some("Renewal order deduction".to_string()),
            )
            .await;
        }

        if let Err(e) = self.repos.order.insert(&order).await {
            tracing::error!(?e, %order_no, "failed to insert reset-traffic order");
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            )));
        }

        self.enqueue_close_task(&order_no).await;

        Ok(ResetTrafficOrderResponse { order_no })
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
