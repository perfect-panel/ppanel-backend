//! `Purchase` (portal) — create order record for a portal user.
//!
//! Portal purchase path: validates plan, applies discount/coupon/fee,
//! inserts the order, then leaves a TODO for the deferred close task.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::config::Config;
use crate::model::dto::order::{PurchaseOrderRequest, PurchaseOrderResponse};
use crate::model::entity::order::Order;
use crate::queue::client::QueueClient;
use crate::queue::types::DEFER_CLOSE_ORDER;
use crate::repository::Repositories;
use crate::service::public::order::calculate_coupon::{calculate_coupon, ensure_enabled};
use crate::service::public::order::calculate_fee::calculate_fee;
use crate::service::public::order::constant::{
    MAX_ORDER_AMOUNT, MAX_QUANTITY, ORDER_STATUS_UNPAID, ORDER_TYPE_SUBSCRIBE,
};
use crate::service::public::order::get_discount::{get_discount, parse_discounts};
use result::code_error::CodeError;
use result::error_code;

use super::tool::generate_trade_no;

pub struct PortalPurchaseService {
    repos: Arc<Repositories>,
    config: Arc<Config>,
    queue: QueueClient,
}

impl PortalPurchaseService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, queue: QueueClient) -> Self {
        Self { repos, config, queue }
    }

    pub async fn purchase(
        &self,
        user_id: i64,
        req: PurchaseOrderRequest,
    ) -> Result<PurchaseOrderResponse, anyhow::Error> {
        let mut quantity = if req.quantity <= 0 { 1 } else { req.quantity };
        if quantity > MAX_QUANTITY {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "quantity exceeds maximum limit",
            )));
        }

        // Single-model guard.
        if self.config.subscribe.single_model {
            let user_subs = self
                .repos
                .user
                .query_user_subscribe(user_id, &[1])
                .await
                .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;
            if !user_subs.is_empty() {
                return Err(anyhow!(CodeError::new_err_code(
                    error_code::USER_SUBSCRIBE_EXIST
                )));
            }
        }

        let mut sub = self
            .repos
            .subscribe
            .find_one(req.subscribe_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        if !sub.sell {
            return Err(anyhow!(CodeError::new_err_code(error_code::ERROR)));
        }
        if sub.inventory == 0 {
            return Err(anyhow!(CodeError::new_err_code(
                error_code::SUBSCRIBE_OUT_OF_STOCK
            )));
        }

        // Quota check.
        if sub.quota > 0 {
            let count = self
                .repos
                .user
                .count_user_subscribes_by_user_and_subscribe(user_id, req.subscribe_id)
                .await
                .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;
            if count >= sub.quota {
                return Err(anyhow!(CodeError::new_err_code(
                    error_code::SUBSCRIBE_QUOTA_LIMIT
                )));
            }
        }

        // Discount ladder.
        let discount = if sub.discount.is_empty() {
            1.0
        } else {
            let tiers = parse_discounts(&sub.discount);
            get_discount(&tiers, quantity)
        };
        let price = sub.unit_price.saturating_mul(quantity);
        let amount = ((price as f64) * discount).round() as i64;
        let discount_amount = price - amount;

        if amount > MAX_ORDER_AMOUNT {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "order amount exceeds maximum limit",
            )));
        }

        // Coupon.
        let mut coupon_discount: i64 = 0;
        if let Some(code) = req.coupon.as_deref().filter(|c| !c.is_empty()) {
            let coupon_info = self
                .repos
                .coupon
                .find_one_by_code(code)
                .await
                .map_err(|e| match e {
                    sqlx::Error::RowNotFound => {
                        anyhow!(CodeError::new_err_code(error_code::COUPON_NOT_EXIST))
                    }
                    _ => anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)),
                })?;
            ensure_enabled(&coupon_info)?;
            coupon_discount = calculate_coupon(amount, &coupon_info);
        }
        let mut amount = amount - coupon_discount;

        // Payment method.
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

        // Fee.
        let mut fee_amount: i64 = 0;
        if amount > 0 {
            fee_amount = calculate_fee(amount, &payment);
            amount += fee_amount;
            if amount > MAX_ORDER_AMOUNT {
                return Err(anyhow!(CodeError::new_err_code_msg(
                    error_code::INVALID_PARAMS,
                    "order amount exceeds maximum limit",
                )));
            }
        }

        // is_new flag.
        let is_new = self
            .repos
            .order
            .is_user_eligible_for_new_order(user_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let now = Utc::now().timestamp_millis();
        let order_no = generate_trade_no();

        let order = Order {
            id: 0,
            parent_id: None,
            user_id,
            order_no: order_no.clone(),
            type_: ORDER_TYPE_SUBSCRIBE,
            quantity,
            price,
            amount,
            gift_amount: 0,
            discount: discount_amount,
            coupon: req.coupon.clone(),
            coupon_discount,
            commission: 0,
            payment_id: payment.id,
            method: payment.platform.clone(),
            fee_amount,
            trade_no: None,
            status: ORDER_STATUS_UNPAID,
            subscribe_id: req.subscribe_id,
            subscribe_token: None,
            is_new,
            created_at: now,
            updated_at: now,
        };

        // Decrement inventory.
        if sub.inventory != -1 {
            sub.inventory -= 1;
            sub.updated_at = now;
            self.repos.subscribe.update(&sub).await.map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_UPDATE_ERROR,
                    &e.to_string(),
                ))
            })?;
        }

        self.repos.order.insert(&order).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            ))
        })?;

        let close_delay = std::time::Duration::from_secs(
            (crate::service::public::order::constant::CLOSE_ORDER_TIME_MINUTES as u64) * 60,
        );
        let close_payload = match serde_json::to_vec(&order_no) {
            Ok(b) => b,
            Err(e) => {
                tracing::error!(%order_no, "failed to serialize close-order payload: {e}");
                return Ok(PurchaseOrderResponse { order_no });
            }
        };
        if let Err(e) = self.queue.enqueue_delayed(DEFER_CLOSE_ORDER, &close_payload, close_delay).await {
            tracing::error!(%order_no, "failed to enqueue defer close-order task: {e}");
        }

        Ok(PurchaseOrderResponse { order_no })
    }
}
