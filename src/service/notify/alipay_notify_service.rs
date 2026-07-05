//! Alipay async-notification handler.
//!
//! Port of `server/internal/logic/notify/alipayNotifyLogic.go`.

use std::sync::Arc;

use anyhow::anyhow;

use payment::alipay::{Config as AlipayConfig, OrderStatus, Provider};
use crate::model::entity::payment::AlipayF2FConfig;
use crate::queue::client::QueueClient;
use crate::queue::types::FORTHWITH_ACTIVATE_ORDER;
use crate::repository::Repositories;

pub struct AlipayNotifyService {
    pub repos: Arc<Repositories>,
}

impl AlipayNotifyService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn handle(&self, token: &str, body: axum::body::Bytes, queue: &QueueClient) -> Result<String, anyhow::Error> {
        // 1. Load payment config.
        let payment = self
            .repos
            .payment
            .find_one_by_token(token)
            .await
            .map_err(|e| anyhow!("load payment config: {e}"))?;

        let cfg: AlipayF2FConfig = serde_json::from_str(&payment.config)
            .map_err(|e| anyhow!("parse alipay config: {e}"))?;

        let provider = Provider::new(AlipayConfig {
            app_id: cfg.app_id,
            private_key: cfg.private_key,
            public_key: cfg.public_key,
            invoice_name: String::new(),
            notify_url: String::new(),
            sandbox: cfg.sandbox,
        })
        .map_err(|e| anyhow!("init alipay provider: {e}"))?;

        // 2. Verify signature — hard failure if invalid.
        let notification = provider
            .decode_notification(&body)
            .map_err(|e| anyhow!("alipay signature verify failed: {e}"))?;

        // 3. Only process TRADE_SUCCESS.
        if notification.status != OrderStatus::Success {
            return Ok("success".into());
        }

        // 4. Load order.
        let order = self
            .repos
            .order
            .find_one_by_order_no(&notification.order_no)
            .await
            .map_err(|e| anyhow!("find order {}: {e}", notification.order_no))?;

        // 5. Enqueue activation task.
        let payload = serde_json::to_vec(&order.id).map_err(|e| anyhow!("serialize payload: {e}"))?;
        queue
            .enqueue(FORTHWITH_ACTIVATE_ORDER, &payload)
            .await
            .map_err(|e| anyhow!("enqueue activate order: {e}"))?;
        tracing::info!(order_no = %order.order_no, task = FORTHWITH_ACTIVATE_ORDER, "enqueued activate order");

        Ok("success".into())
    }
}
