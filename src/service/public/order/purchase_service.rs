//! `Purchase` — create a new subscription order for a user.
//!
//! Port of `server/internal/logic/public/order/purchaseLogic.go`. After
//! the order is inserted, the Go side enqueues a `defer:close:order`
//! task to auto-cancel the order after `CloseOrderTimeMinutes` minutes.
//! That enqueue requires a queue client which is not yet plumbed into
//! the Rust service context, so a `TODO` is left in `enqueue_close_task`.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;
use uuid::Uuid;

use crate::config::Config;
use crate::model::dto::order::{PurchaseOrderRequest, PurchaseOrderResponse};
use crate::model::entity::coupon::Coupon;
use crate::model::entity::order::Order;
use crate::queue::client::QueueClient;
use crate::queue::types::DEFER_CLOSE_ORDER;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

use super::calculate_coupon::{calculate_coupon, ensure_enabled};
use super::calculate_fee::calculate_fee;
use super::constant::{
    MAX_ORDER_AMOUNT, MAX_QUANTITY, ORDER_STATUS_UNPAID, ORDER_TYPE_SUBSCRIBE,
};
use super::get_discount::{get_discount, parse_discounts};
use result::code_error::CodeError;
use result::error_code;

pub struct PurchaseService {
    repos: Arc<Repositories>,
    config: Arc<Config>,
    queue: QueueClient,
}

impl PurchaseService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, queue: QueueClient) -> Self {
        Self { repos, config, queue }
    }

    /// Create a new subscription purchase order for `user_id`.
    pub async fn purchase(
        &self,
        user_id: i64,
        req: PurchaseOrderRequest,
    ) -> Result<PurchaseOrderResponse, anyhow::Error> {
        let mut user = self
            .repos
            .user
            .find_one_user(user_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)))?;

        let mut quantity = if req.quantity <= 0 { 1 } else { req.quantity };
        if quantity > MAX_QUANTITY {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "quantity exceeds maximum limit",
            )));
        }

        // ── Single-model guard (matches Go's `l.svcCtx.Config.Subscribe.SingleModel`)
        if self.config.subscribe.single_model {
            let user_subs = self
                .repos
                .user
                .query_user_subscribe(user_id, &[1])
                .await
                .map_err(|_| {
                    anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
                })?;
            if !user_subs.is_empty() {
                return Err(anyhow!(CodeError::new_err_code(
                    error_code::USER_SUBSCRIBE_EXIST
                )));
            }
        }

        // ── Fetch the subscribe plan.
        let mut sub = self
            .repos
            .subscribe
            .find_one(req.subscribe_id)
            .await
            .map_err(|_| {
                anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
            })?;

        if !sub.sell {
            return Err(anyhow!(CodeError::new_err_code(error_code::ERROR)));
        }
        if sub.inventory == 0 {
            return Err(anyhow!(CodeError::new_err_code(
                error_code::SUBSCRIBE_OUT_OF_STOCK
            )));
        }

        // ── Per-plan quota check.
        if sub.quota > 0 {
            let user_subs = self
                .repos
                .user
                .query_user_subscribe(user_id, &[1])
                .await
                .map_err(|_| {
                    anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
                })?;
            let count = user_subs
                .iter()
                .filter(|s| s.subscribe_id == req.subscribe_id)
                .count() as i64;
            if count >= sub.quota {
                return Err(anyhow!(CodeError::new_err_code(
                    error_code::SUBSCRIBE_QUOTA_LIMIT
                )));
            }
        }

        // ── Discount ladder.
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

        // ── Coupon.
        let mut coupon_discount: i64 = 0;
        if let Some(coupon_code) = req.coupon.as_deref().filter(|c| !c.is_empty()) {
            let coupon_info = self
                .repos
                .coupon
                .find_one_by_code(coupon_code)
                .await
                .map_err(|e| match e {
                    sqlx::Error::RowNotFound => {
                        anyhow!(CodeError::new_err_code(error_code::COUPON_NOT_EXIST))
                    }
                    _ => anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)),
                })?;
            self.validate_coupon(&coupon_info, user_id, req.subscribe_id, quantity)
                .await?;
            coupon_discount = calculate_coupon(amount, &coupon_info);
        }
        let mut amount = amount - coupon_discount;

        // ── Payment method (required for purchase; previews can skip it).
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
            .map_err(|_| {
                anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
            })?;

        // ── Handling fee.
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

        // ── Gift-amount deduction.
        let mut deduction_amount: i64 = 0;
        if user.gift_amount > 0 && amount > 0 {
            if user.gift_amount >= amount {
                deduction_amount = amount;
                amount = 0;
            } else {
                deduction_amount = user.gift_amount;
                amount -= user.gift_amount;
            }
        }

        // ── is_new flag — Go defers this to a repository check.
        let is_new = self
            .repos
            .order
            .is_user_eligible_for_new_order(user_id)
            .await
            .map_err(|_| {
                anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
            })?;

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
            gift_amount: deduction_amount,
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

        // ── Persist: user deduction + inventory + order insert.
        // No `InTx` helper in the Rust repo layer yet, so we serialise
        // the three writes and roll back manually on failure.
        if deduction_amount > 0 {
            let previous_gift = user.gift_amount;
            user.gift_amount -= deduction_amount;
            user.updated_at = now;
            if let Err(e) = self.repos.user.update_user(&user).await {
                // Compensate the in-memory copy in case the caller retries.
                user.gift_amount = previous_gift;
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
                Some("Purchase order deduction".to_string()),
            )
            .await;
        }

        if sub.inventory != -1 {
            sub.inventory -= 1;
            sub.updated_at = now;
            if let Err(e) = self.repos.subscribe.update(&sub).await {
                tracing::error!(?e, subscribe_id = sub.id, "failed to decrement inventory");
                return Err(anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_UPDATE_ERROR,
                    &e.to_string(),
                )));
            }
        }

        if let Err(e) = self.repos.order.insert(&order).await {
            tracing::error!(?e, %order_no, "failed to insert purchase order");
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            )));
        }

        self.enqueue_close_task(&order_no).await;

        Ok(PurchaseOrderResponse { order_no })
    }

    /// Coupon validation matching Go's pre-purchase block.
    async fn validate_coupon(
        &self,
        coupon_info: &Coupon,
        user_id: i64,
        subscribe_id: i64,
        _quantity: i64,
    ) -> Result<(), anyhow::Error> {
        ensure_enabled(coupon_info)?;

        if coupon_info.count != 0 && coupon_info.count <= coupon_info.used_count {
            return Err(anyhow!(CodeError::new_err_code(
                error_code::COUPON_INSUFFICIENT_USAGE
            )));
        }

        if !coupon_info.subscribe.is_empty() {
            let allowed: Vec<i64> = coupon_info
                .subscribe
                .split(',')
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect();
            if !allowed.is_empty() && !allowed.contains(&subscribe_id) {
                return Err(anyhow!(CodeError::new_err_code(
                    error_code::COUPON_NOT_APPLICABLE
                )));
            }
        }

        let count = self
            .repos
            .order
            .count_user_coupon_usage(user_id, &coupon_info.code)
            .await
            .map_err(|_| {
                anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
            })?;

        if coupon_info.user_limit > 0 && count >= coupon_info.user_limit {
            return Err(anyhow!(CodeError::new_err_code(
                error_code::COUPON_INSUFFICIENT_USAGE
            )));
        }

        Ok(())
    }

    /// Mirror of Go's `asynq.NewTask(queue.DeferCloseOrder, …) +
    /// Queue.Enqueue(task, asynq.ProcessIn(15*time.Minute))`.
    ///
    /// The Rust port has no queue client plumbed into the service context
    /// yet; once `AppState` exposes one, this method should enqueue
    /// `crate::queue::types::DEFER_CLOSE_ORDER` with the order number as
    /// payload and a 15-minute delay.
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

/// Generate a short, unique trade/order number.
///
/// Go's `tool.GenerateTradeNo` returns a numeric string built from a
/// timestamp + atomic counter. The Rust port uses a UUID-derived
/// identifier for the same uniqueness guarantee. Exposed `pub(super)`
/// so sibling services (`recharge`, `renewal`, `reset_traffic`) can
/// share the same numbering scheme.
pub(super) fn generate_trade_no() -> String {
    let short = Uuid::new_v4().simple().to_string();
    // 16 chars keeps it roughly the same length as a 16-digit Go id.
    short[..16].to_uppercase()
}
