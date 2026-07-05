//! `PrePurchaseOrder` — price preview without DB write.
//!
//! Port of the portal pre-purchase logic. Validates the plan, applies
//! discount ladder, applies coupon (if any), and adds payment fee.
//! No records are inserted.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::order::{PrePurchaseOrderRequest, PrePurchaseOrderResponse};
use crate::repository::Repositories;
use crate::service::public::order::calculate_coupon::{calculate_coupon, ensure_enabled};
use crate::service::public::order::calculate_fee::calculate_fee;
use crate::service::public::order::get_discount::{get_discount, parse_discounts};
use result::code_error::CodeError;
use result::error_code;

pub struct PrePurchaseOrderService {
    repos: Arc<Repositories>,
}

impl PrePurchaseOrderService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn pre_purchase(
        &self,
        req: PrePurchaseOrderRequest,
    ) -> Result<PrePurchaseOrderResponse, anyhow::Error> {
        let quantity = if req.quantity <= 0 { 1 } else { req.quantity };

        let sub = self
            .repos
            .subscribe
            .find_one(req.subscribe_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        if !sub.sell {
            return Err(anyhow!(CodeError::new_err_code(error_code::ERROR)));
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

        // Coupon.
        let mut coupon_discount: i64 = 0;
        let coupon_str = req.coupon.clone().unwrap_or_default();
        if !coupon_str.is_empty() {
            let coupon_info = self
                .repos
                .coupon
                .find_one_by_code(&coupon_str)
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

        // Payment fee.
        let mut fee_amount: i64 = 0;
        if let Some(payment_id) = req.payment {
            let payment = self
                .repos
                .payment
                .find_one(payment_id)
                .await
                .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;
            if amount > 0 {
                fee_amount = calculate_fee(amount, &payment);
                amount += fee_amount;
            }
        }

        Ok(PrePurchaseOrderResponse {
            price,
            amount,
            discount: discount_amount,
            coupon: coupon_str,
            coupon_discount,
            fee_amount,
        })
    }
}
