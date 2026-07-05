//! Stripe webhook handler.
//!
//! Port of `server/internal/logic/notify/stripeNotifyLogic.go`.

use std::sync::Arc;

use anyhow::anyhow;

use payment::stripe::{Config as StripeConfig, Provider};
use crate::model::entity::payment::StripeConfig as StripeEntityConfig;
use crate::queue::client::QueueClient;
use crate::queue::types::FORTHWITH_ACTIVATE_ORDER;
use crate::repository::Repositories;

pub struct StripeNotifyService {
    pub repos: Arc<Repositories>,
}

impl StripeNotifyService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn handle(
        &self,
        token: &str,
        payload: axum::body::Bytes,
        signature: &str,
        queue: &QueueClient,
    ) -> Result<String, anyhow::Error> {
        // 1. Load payment config.
        let payment = self
            .repos
            .payment
            .find_one_by_token(token)
            .await
            .map_err(|e| anyhow!("load payment config: {e}"))?;

        let cfg: StripeEntityConfig = serde_json::from_str(&payment.config)
            .map_err(|e| anyhow!("parse stripe config: {e}"))?;

        let provider = Provider::new(StripeConfig {
            public_key: cfg.public_key,
            secret_key: cfg.secret_key,
            webhook_secret: cfg.webhook_secret,
        });

        // 2. Verify webhook signature — hard failure if invalid.
        let notification = provider
            .parse_notify(&payload, signature)
            .map_err(|e| anyhow!("stripe signature verify failed: {e}"))?;

        // 3. Only process payment_intent.succeeded / checkout.session.completed.
        let is_success = notification.event_type == "payment_intent.succeeded"
            || notification.event_type == "checkout.session.completed";

        if !is_success {
            return Ok("ok".into());
        }

        if notification.order_no.is_empty() {
            return Err(anyhow!("stripe notification missing order_no in metadata"));
        }

        // 4. Load order.
        let order = self
            .repos
            .order
            .find_one_by_order_no(&notification.order_no)
            .await
            .map_err(|e| anyhow!("find order {}: {e}", notification.order_no))?;

        // 5. Enqueue activation.
        let payload_bytes = serde_json::to_vec(&order.id).map_err(|e| anyhow!("serialize payload: {e}"))?;
        queue
            .enqueue(FORTHWITH_ACTIVATE_ORDER, &payload_bytes)
            .await
            .map_err(|e| anyhow!("enqueue activate order: {e}"))?;
        tracing::info!(
            order_no = %order.order_no,
            trade_no = %notification.trade_no,
            task = FORTHWITH_ACTIVATE_ORDER,
            "enqueued activate order"
        );

        Ok("ok".into())
    }
}
