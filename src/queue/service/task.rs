use std::sync::Arc;

use chrono::Utc;
use serde::Deserialize;

use crate::config::Config;
use crate::model::entity::log::{GIFT_TYPE_INCREASE, RESET_SUBSCRIBE_TYPE_QUOTA};
use crate::model::entity::task::{QuotaContent, QuotaScope, Task};
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

/// Port of `server/queue/logic/task/quotaLogic.go`.
///
/// Payload: raw bytes that are a decimal task ID string.
/// The actual work parameters live in `tasks.scope` / `tasks.content` columns.
pub struct QuotaTaskLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl QuotaTaskLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self, raw: &[u8]) -> anyhow::Result<()> {
        // ── 1. Parse task ID ────────────────────────────────────────────────
        let task_id: i64 = match std::str::from_utf8(raw)
            .ok()
            .and_then(|s| s.trim().parse().ok())
        {
            Some(id) => id,
            None => {
                tracing::error!("[QuotaTaskLogic] invalid payload: {:?}", raw);
                return Ok(());
            }
        };

        // ── 2. Fetch task record ────────────────────────────────────────────
        let mut task = match self.repos.task.find_one(task_id).await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("[QuotaTaskLogic] find_one({task_id}): {e}");
                return Ok(());
            }
        };

        if task.status != 0 {
            tracing::info!(
                "[QuotaTaskLogic] task {task_id} already processed (status={})",
                task.status
            );
            return Ok(());
        }

        // ── 3. Parse scope + content ────────────────────────────────────────
        let scope: QuotaScope = match task
            .scope
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
        {
            Some(s) => s,
            None => {
                tracing::error!("[QuotaTaskLogic] failed to parse scope for task {task_id}");
                return Ok(());
            }
        };

        let content: QuotaContent = match task
            .content
            .as_deref()
            .and_then(|s| serde_json::from_str(s).ok())
        {
            Some(c) => c,
            None => {
                tracing::error!("[QuotaTaskLogic] failed to parse content for task {task_id}");
                return Ok(());
            }
        };

        // ── 4. Resolve subscriber IDs from scope ────────────────────────────
        let sub_ids = self.resolve_subscriber_ids(&scope).await;

        // ── 5. Fetch subscribe records ──────────────────────────────────────
        let subscribes = match self.repos.user.find_subscribes_by_ids(&sub_ids).await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("[QuotaTaskLogic] find_subscribes_by_ids: {e}");
                return Ok(());
            }
        };

        // ── 6. Process each subscribe ───────────────────────────────────────
        let now_ms = Utc::now().timestamp_millis();
        let mut errors: Vec<String> = Vec::new();

        for mut sub in subscribes {
            let mut updated = false;

            // Extend expiry
            if let Some(days) = content.days {
                if days != 0 {
                    let base = if sub.expire_time == 0 || sub.expire_time < now_ms {
                        now_ms
                    } else {
                        sub.expire_time
                    };
                    sub.expire_time =
                        chrono::DateTime::<Utc>::from_timestamp_millis(base)
                            .unwrap_or_else(Utc::now)
                            .checked_add_signed(chrono::Duration::days(days))
                            .unwrap_or_else(Utc::now)
                            .timestamp_millis();
                    if sub.expire_time > now_ms && sub.status != 1 {
                        sub.status = 1;
                    }
                    updated = true;
                }
            }

            // Reset traffic
            if content.reset_traffic {
                sub.download = 0;
                sub.upload = 0;
                updated = true;
                Telemetry::reset_subscribe(
                    &self.repos,
                    sub.user_id,
                    RESET_SUBSCRIBE_TYPE_QUOTA,
                    None,
                )
                .await;
            }

            // Gift amount
            if let (Some(gift_type), Some(gift_value)) = (content.gift_type, content.gift_value) {
                if gift_value != 0 {
                    if let Err(e) = self
                        .process_gift(sub.user_id, sub.id, sub.subscribe_id, gift_type, gift_value)
                        .await
                    {
                        tracing::error!(
                            "[QuotaTaskLogic] process_gift for subscribe {}: {e}",
                            sub.id
                        );
                        errors.push(format!("subscribe {}: gift error: {e}", sub.id));
                    }
                }
            }

            if updated {
                sub.updated_at = now_ms;
                if let Err(e) = self.repos.user.update_subscribe(&sub).await {
                    tracing::error!("[QuotaTaskLogic] update_subscribe({}): {e}", sub.id);
                    errors.push(format!("subscribe {}: update error: {e}", sub.id));
                }
            }
        }

        // ── 7. Finalize task record ─────────────────────────────────────────
        let all_failed = !errors.is_empty() && errors.len() >= sub_ids.len();
        task.status = if all_failed { 3 } else { 2 };
        task.current = sub_ids.len() as i64;
        if !errors.is_empty() {
            task.errors = serde_json::to_string(&errors).ok();
        }
        task.updated_at = now_ms;

        if let Err(e) = self.repos.task.update(&task).await {
            tracing::error!("[QuotaTaskLogic] update task {task_id}: {e}");
        }

        Ok(())
    }

    // ── helper: resolve subscriber IDs from scope ─────────────────────────────

    async fn resolve_subscriber_ids(&self, scope: &QuotaScope) -> Vec<i64> {
        // Direct list wins
        if !scope.recipients.is_empty() {
            return scope.recipients.clone();
        }

        // Filter by active/expired status
        use crate::repository::user::SubscribeFilter;
        let filter = SubscribeFilter {
            subscribers: scope.subscribers.clone(),
            is_active: scope.is_active,
            start_time: scope.start_time,
            end_time: scope.end_time,
        };

        match self.repos.user.query_subscribe_ids_by_filter(&filter).await {
            Ok(ids) => ids,
            Err(e) => {
                tracing::error!("[QuotaTaskLogic] query_subscribe_ids_by_filter: {e}");
                vec![]
            }
        }
    }

    // ── helper: apply gift to user balance ────────────────────────────────────

    async fn process_gift(
        &self,
        user_id: i64,
        subscribe_id: i64,
        plan_subscribe_id: i64,
        gift_type: i16,
        gift_value: i64,
    ) -> anyhow::Result<()> {
        let mut user = self.repos.user.find_one_user(user_id).await?;

        let gift_amount: i64 = match gift_type {
            1 => gift_value,
            2 => {
                // Percentage of plan unit price
                let plan = self.repos.subscribe.find_one(plan_subscribe_id).await?;
                if plan.unit_price > 0 {
                    (plan.unit_price as f64 * (gift_value as f64 / 100.0)) as i64
                } else {
                    0
                }
            }
            other => {
                return Err(anyhow::anyhow!("invalid gift_type {other}"));
            }
        };

        if gift_amount <= 0 {
            return Ok(());
        }

        user.gift_amount += gift_amount;
        user.updated_at = Utc::now().timestamp_millis();
        let updated = self.repos.user.update_user(&user).await?;

        Telemetry::gift(
            &self.repos,
            user_id,
            GIFT_TYPE_INCREASE,
            "",
            subscribe_id,
            gift_amount,
            updated.gift_amount,
            Some("Quota task gift".to_string()),
        )
        .await;

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RateLogic — port of `server/queue/logic/task/rateLogic.go`
// Fetches the current exchange rate for the site currency and caches it.
// ─────────────────────────────────────────────────────────────────────────────

pub struct RateLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl RateLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    /// Fetch exchange rate from apilayer.com and cache it in Redis.
    ///
    /// Skips silently when `Currency.AccessKey` is not configured.
    pub async fn execute(&self) -> anyhow::Result<()> {
        let access_key = &self.config.currency.access_key;
        if access_key.is_empty() {
            tracing::debug!("[RateLogic] skip: no Currency.AccessKey configured");
            return Ok(());
        }

        let from = &self.config.currency.unit;
        if from.is_empty() || from.to_uppercase() == "CNY" {
            // Already in base currency — rate is 1.0, nothing to fetch.
            tracing::debug!("[RateLogic] skip: currency unit is CNY or empty");
            return Ok(());
        }

        match crate::exchange_rate::convert(from, "CNY", 1.0, access_key).await {
            Ok(rate) => {
                tracing::info!(
                    "[RateLogic] exchange rate {from}→CNY = {rate:.6}"
                );
                // Persist rate to system config table.
                let _ = self
                    .repos
                    .system
                    .update_value_by_category_key("currency", "exchange_rate", &rate.to_string())
                    .await
                    .map_err(|e| tracing::warn!("[RateLogic] persist rate failed: {e}"));
            }
            Err(e) => {
                tracing::error!("[RateLogic] fetch exchange rate failed: {e}");
                return Err(e);
            }
        }
        Ok(())
    }
}
