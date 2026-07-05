//! `PreCreateOrder` — order price preview without persistence.
//!
//! Port of `server/internal/logic/public/order/preCreateOrderLogic.go`.
//! Validates the plan, fetches the coupon (if any), applies the discount
//! ladder, adds the payment handling fee, then deducts the user's gift
//! balance — exactly mirroring Go's pipeline. The result is a
//! [`PreOrderResponse`] for the frontend preview screen.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::order::{PreOrderResponse, PurchaseOrderRequest};
use crate::model::entity::coupon::Coupon;
use crate::repository::Repositories;

use super::calculate_coupon::{calculate_coupon, ensure_enabled};
use super::calculate_fee::calculate_fee;
use super::get_discount::{get_discount, parse_discounts};
use result::code_error::CodeError;
use result::error_code;

pub struct PreCreateOrderService {
    repos: Arc<Repositories>,
}

impl PreCreateOrderService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    /// Compute the price preview for the user identified by `user_id`.
    pub async fn pre_create(
        &self,
        user_id: i64,
        req: PurchaseOrderRequest,
    ) -> Result<PreOrderResponse, anyhow::Error> {
        // The handler is expected to have already extracted `user_id` from
        // the auth context; we still need the full User record for the
        // `gift_amount` deduction. The auth middleware guarantees the
        // user exists and is enabled.
        let user = self
            .repos
            .user
            .find_one_user(user_id)
            .await
            .map_err(|_| {
                anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
            })?;

        // Normalise quantity — Go sets it to 1 when ≤ 0.
        let quantity = if req.quantity <= 0 { 1 } else { req.quantity };

        // 1. Fetch the subscribe plan.
        let sub = self
            .repos
            .subscribe
            .find_one(req.subscribe_id)
            .await
            .map_err(|_| {
                anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
            })?;

        // 2. Optional per-user quota check (matches Go preCreateOrderLogic.go:60-68).
        if sub.quota > 0 {
            let count = self
                .repos
                .user
                .count_user_subscribes_by_user_and_subscribe(user_id, req.subscribe_id)
                .await
                .map_err(|_| {
                    anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
                })?;
            if count >= sub.quota {
                return Err(anyhow!(CodeError::new_err_code(
                    error_code::SUBSCRIBE_QUOTA_LIMIT
                )));
            }
        }

        // 3. Quantity-based discount multiplier.
        let discount = if sub.discount.is_empty() {
            1.0
        } else {
            let tiers = parse_discounts(&sub.discount);
            get_discount(&tiers, quantity)
        };

        let price = sub.unit_price.saturating_mul(quantity);
        let amount = ((price as f64) * discount).round() as i64;
        let discount_amount = price - amount;

        // 4. Optional coupon.
        let mut coupon_amount: i64 = 0;
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

            self.validate_coupon(&coupon_info, user_id, req.subscribe_id)
                .await?;
            coupon_amount = calculate_coupon(amount, &coupon_info);
        }
        let mut amount = amount - coupon_amount;

        // 5. Payment handling fee.
        let mut fee_amount: i64 = 0;
        if let Some(payment_id) = req.payment {
            let payment = self
                .repos
                .payment
                .find_one(payment_id)
                .await
                .map_err(|_| {
                    anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR))
                })?;
            if amount > 0 {
                fee_amount = calculate_fee(amount, &payment);
                amount += fee_amount;
            }
        }

        // 6. Gift-amount deduction (Go's `deductionAmount`).
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

        // Pre-create is a read-only flow — no telemetry write needed.

        Ok(PreOrderResponse {
            price,
            amount,
            discount: discount_amount,
            gift_amount: deduction_amount,
            coupon: req.coupon.unwrap_or_default(),
            coupon_discount: coupon_amount,
            fee_amount,
        })
    }

    /// Verify the coupon is enabled, has remaining quota, and applies to
    /// the chosen plan. Mirrors Go's `if err := ensureCouponEnabled(…); …`
    /// block in preCreateOrderLogic.go.
    async fn validate_coupon(
        &self,
        coupon_info: &Coupon,
        user_id: i64,
        subscribe_id: i64,
    ) -> Result<(), anyhow::Error> {
        ensure_enabled(coupon_info)?;

        if coupon_info.count > 0 && coupon_info.count <= coupon_info.used_count {
            return Err(anyhow!(CodeError::new_err_code(
                error_code::COUPON_ALREADY_USED
            )));
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

        // `coupon_info.subscribe` is a comma-separated list of subscribe
        // ids the coupon can apply to. An empty string means "all".
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

        Ok(())
    }
}
