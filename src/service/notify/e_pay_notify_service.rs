//! ePay async-notification handler.
//!
//! Port of `server/internal/logic/notify/ePayNotifyLogic.go`.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;

use payment::epay::{Config as EPayConfig, Provider};
use crate::model::entity::payment::EPayConfig as EPayEntityConfig;
use crate::queue::client::QueueClient;
use crate::queue::types::FORTHWITH_ACTIVATE_ORDER;
use crate::repository::Repositories;

pub struct EPayNotifyService {
    pub repos: Arc<Repositories>,
}

impl EPayNotifyService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn handle(
        &self,
        token: &str,
        params: HashMap<String, String>,
        queue: &QueueClient,
    ) -> Result<String, anyhow::Error> {
        // 1. Load payment config.
        let payment = self
            .repos
            .payment
            .find_one_by_token(token)
            .await
            .map_err(|e| anyhow!("load payment config: {e}"))?;

        let cfg: EPayEntityConfig = serde_json::from_str(&payment.config)
            .map_err(|e| anyhow!("parse epay config: {e}"))?;

        let provider = Provider::new(EPayConfig {
            pid: cfg.pid,
            url: cfg.url,
            key: cfg.key,
            pay_type: cfg.type_,
        });

        // 2. Verify MD5 signature — hard failure if invalid.
        if !provider.verify_sign(&params) {
            return Err(anyhow!("epay signature verification failed"));
        }

        // 3. Only process TRADE_SUCCESS.
        let trade_status = params.get("trade_status").map(String::as_str).unwrap_or("");
        if trade_status != "TRADE_SUCCESS" {
            return Ok("success".into());
        }

        let order_no = params
            .get("out_trade_no")
            .ok_or_else(|| anyhow!("missing out_trade_no"))?;

        // 4. Load order.
        let order = self
            .repos
            .order
            .find_one_by_order_no(order_no)
            .await
            .map_err(|e| anyhow!("find order {order_no}: {e}"))?;

        // 5. Enqueue activation.
        let payload = serde_json::to_vec(&order.id).map_err(|e| anyhow!("serialize payload: {e}"))?;
        queue
            .enqueue(FORTHWITH_ACTIVATE_ORDER, &payload)
            .await
            .map_err(|e| anyhow!("enqueue activate order: {e}"))?;
        tracing::info!(order_no = %order.order_no, task = FORTHWITH_ACTIVATE_ORDER, "enqueued activate order");

        Ok("success".into())
    }
}
