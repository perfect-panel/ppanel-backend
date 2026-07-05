//! `Renewal` — extend an existing user-subscription by creating a renewal
//! order against its parent plan.
//!
//! Port of `server/internal/logic/public/order/renewalLogic.go`.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::order::{RenewalOrderRequest, RenewalOrderResponse};
use crate::model::entity::coupon::Coupon;
use crate::model::entity::order::Order;
use crate::queue::client::QueueClient;
use crate::queue::types::DEFER_CLOSE_ORDER;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

use super::calculate_coupon::{calculate_coupon, ensure_enabled};
use super::calculate_fee::calculate_fee;
use super::constant::{MAX_ORDER_AMOUNT, MAX_QUANTITY, ORDER_STATUS_UNPAID, ORDER_TYPE_RENEWAL};
use super::get_discount::{get_discount, parse_discounts};
use super::purchase_service::generate_trade_no;
use result::code_error::CodeError;
use result::error_code;

pub struct RenewalService {
    repos: Arc<Repositories>,
    queue: QueueClient,
}

impl RenewalService {
    pub fn new(repos: Arc<Repositories>, queue: QueueClient) -> Self {
        Self { repos, queue }
    }

    pub async fn renewal(
        &self,
        user_id: i64,
        req: RenewalOrderRequest,
    ) -> Result<RenewalOrderResponse, anyhow::Error> {
        let mut user = self
            .repos
            .user
            .find_one_user(user_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)))?;

        let quantity = if req.quantity <= 0 { 1 } else { req.quantity };
        if quantity > MAX_QUANTITY {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "quantity exceeds maximum limit",
            )));
        }

        let user_sub = self
            .repos
            .user
            .find_one_user_subscribe(req.user_subscribe_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let sub = self
            .repos
            .subscribe
            .find_one(user_sub.subscribe_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        if !sub.sell {
            return Err(anyhow!(CodeError::new_err_code(error_code::ERROR)));
        }

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
            self.validate_coupon(&coupon_info, user_id, sub.id).await?;
            coupon_discount = calculate_coupon(amount, &coupon_info);
        }
        let mut amount = amount - coupon_discount;

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

        // Deduct gift amount (Go subtracts from amount first, then fee).
        let mut deduction_amount: i64 = 0;
        if user.gift_amount > 0 {
            if user.gift_amount >= amount {
                deduction_amount = amount;
                user.gift_amount -= deduction_amount;
                amount = 0;
            } else {
                deduction_amount = user.gift_amount;
                amount -= user.gift_amount;
                user.gift_amount = 0;
            }
        }

        let mut fee_amount: i64 = 0;
        if amount > 0 {
            fee_amount = calculate_fee(amount, &payment);
        }
        amount += fee_amount;
        if amount > MAX_ORDER_AMOUNT {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::INVALID_PARAMS,
                "order amount exceeds maximum limit",
            )));
        }

        let now = Utc::now().timestamp_millis();
        let order_no = generate_trade_no();
        let order = Order {
            id: 0,
            parent_id: Some(user_sub.order_id),
            user_id,
            order_no: order_no.clone(),
            type_: ORDER_TYPE_RENEWAL,
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
            subscribe_id: user_sub.subscribe_id,
            subscribe_token: Some(user_sub.token.clone()),
            is_new: false,
            created_at: now,
            updated_at: now,
        };

        if deduction_amount > 0 {
            user.updated_at = now;
            if let Err(e) = self.repos.user.update_user(&user).await {
                tracing::error!(?e, user_id, "failed to deduct gift_amount on renewal");
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
            tracing::error!(?e, %order_no, "failed to insert renewal order");
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_INSERT_ERROR,
                &e.to_string(),
            )));
        }

        self.enqueue_close_task(&order_no).await;

        Ok(RenewalOrderResponse { order_no })
    }

    async fn validate_coupon(
        &self,
        coupon_info: &Coupon,
        user_id: i64,
        subscribe_id: i64,
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
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        if count >= coupon_info.user_limit {
            return Err(anyhow!(CodeError::new_err_code(
                error_code::COUPON_INSUFFICIENT_USAGE
            )));
        }

        Ok(())
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
