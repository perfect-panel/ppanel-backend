//! `PurchaseCheckout` — return payment URL / stub per payment platform.
//!
//! Port of the portal checkout logic. Looks up the order and its payment
//! method, then delegates to the per-platform checkout stub.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::order::{CheckoutOrderRequest, CheckoutOrderResponse};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct PurchaseCheckoutService {
    repos: Arc<Repositories>,
}

impl PurchaseCheckoutService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn checkout(
        &self,
        req: CheckoutOrderRequest,
    ) -> Result<CheckoutOrderResponse, anyhow::Error> {
        let order = self
            .repos
            .order
            .find_one_by_order_no(&req.order_no)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    anyhow!(CodeError::new_err_code(error_code::ORDER_NOT_EXIST))
                }
                _ => anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)),
            })?;

        let payment = self
            .repos
            .payment
            .find_one(order.payment_id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        // If amount is 0 the order is already paid via gift balance — no
        // external checkout URL is needed.
        if order.amount == 0 {
            return Ok(CheckoutOrderResponse {
                type_: payment.platform.clone(),
                checkout_url: None,
                stripe: None,
            });
        }

        // Per-platform stub — full integration will be added when the
        // `payment` crate is wired up to each provider's API.
        let checkout_url = match payment.platform.as_str() {
            "stripe" => {
                // TODO: create Stripe PaymentIntent and return client_secret
                tracing::warn!(order_no = %req.order_no, "stripe checkout not yet implemented");
                None
            }
            "alipay_f2f" => {
                // TODO: call Alipay Face-to-Face API
                tracing::warn!(order_no = %req.order_no, "alipay_f2f checkout not yet implemented");
                None
            }
            "epay" => {
                // TODO: build EPay redirect URL
                tracing::warn!(order_no = %req.order_no, "epay checkout not yet implemented");
                None
            }
            platform => {
                tracing::warn!(
                    order_no = %req.order_no,
                    %platform,
                    "unknown payment platform for checkout",
                );
                None
            }
        };

        Ok(CheckoutOrderResponse {
            type_: payment.platform,
            checkout_url,
            stripe: None,
        })
    }
}
