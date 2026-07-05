use std::sync::Arc;

use chrono::Utc;

use crate::config::Config;
use crate::repository::Repositories;

/// Port of `server/queue/logic/subscription/checkSubscriptionLogic.go`.
///
/// Two passes:
///  1. Traffic-exceeded subscribes → mark status=2
///  2. Expired subscribes → mark status=3
///
/// For each affected subscribe, enqueue a SendEmail notification if the user
/// has an email auth method (best-effort, errors logged and skipped).
pub struct CheckSubscriptionLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl CheckSubscriptionLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let now_ms = Utc::now().timestamp_millis();

        // ── Pass 1: traffic exceeded ─────────────────────────────────────────
        self.handle_traffic_exceeded(now_ms).await;

        // ── Pass 2: expired ──────────────────────────────────────────────────
        self.handle_expired(now_ms).await;

        Ok(())
    }

    // ── traffic exceeded ─────────────────────────────────────────────────────

    async fn handle_traffic_exceeded(&self, now_ms: i64) {
        let list = match self.repos.user.find_traffic_exceeded_subscribes().await {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("[CheckSubscription/Traffic] find_traffic_exceeded_subscribes: {e}");
                return;
            }
        };

        if list.is_empty() {
            tracing::info!("[CheckSubscription/Traffic] no traffic-exceeded subscribes");
            return;
        }

        let ids: Vec<i64> = list.iter().map(|s| s.id).collect();

        if let Err(e) = self
            .repos
            .user
            .mark_subscribes_finished(&ids, 2, now_ms)
            .await
        {
            tracing::error!("[CheckSubscription/Traffic] mark_subscribes_finished: {e}");
            return;
        }

        tracing::info!(
            "[CheckSubscription/Traffic] marked {} subscribes finished (traffic)",
            ids.len()
        );

        // Enqueue notification emails (best-effort)
        for sub in &list {
            self.send_notification_email(sub.id, sub.user_id, "traffic").await;
        }
    }

    // ── expired ──────────────────────────────────────────────────────────────

    async fn handle_expired(&self, now_ms: i64) {
        let list = match self.repos.user.find_expired_subscribes(now_ms).await {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("[CheckSubscription/Expire] find_expired_subscribes: {e}");
                return;
            }
        };

        if list.is_empty() {
            tracing::info!("[CheckSubscription/Expire] no expired subscribes");
            return;
        }

        let ids: Vec<i64> = list.iter().map(|s| s.id).collect();

        if let Err(e) = self
            .repos
            .user
            .mark_subscribes_finished(&ids, 3, now_ms)
            .await
        {
            tracing::error!("[CheckSubscription/Expire] mark_subscribes_finished: {e}");
            return;
        }

        tracing::info!(
            "[CheckSubscription/Expire] marked {} subscribes finished (expired)",
            ids.len()
        );

        for sub in &list {
            self.send_notification_email(sub.id, sub.user_id, "expired").await;
        }
    }

    // ── notification helper ───────────────────────────────────────────────────

    async fn send_notification_email(&self, subscribe_id: i64, user_id: i64, kind: &str) {
        // Look up the user's email auth method
        let auth = match self.repos.user.find_auth_method_by_user_id("email", user_id).await {
            Ok(Some(a)) => a,
            Ok(None) => {
                tracing::info!(
                    "[CheckSubscription] user {user_id} has no email auth method, skipping"
                );
                return;
            }
            Err(e) => {
                tracing::error!(
                    "[CheckSubscription] find_auth_method_by_user_id(user={user_id}): {e}"
                );
                return;
            }
        };

        let to = auth.auth_identifier;

        // Build the SendEmail payload matching Go `queue/types.SendEmailPayload`
        let (email_type, subject) = if kind == "expired" {
            (3i16, "Subscription Expired")
        } else {
            (4i16, "Subscription Traffic Exceeded")
        };

        let content = serde_json::json!({
            "SiteLogo": self.config.site.site_logo,
            "SiteName": self.config.site.site_name,
        });

        let email_payload = serde_json::json!({
            "type":    email_type,
            "email":   to,
            "subject": subject,
            "content": content,
        });

        let raw = match serde_json::to_vec(&email_payload) {
            Ok(b) => b,
            Err(e) => {
                tracing::error!("[CheckSubscription] serialise email payload: {e}");
                return;
            }
        };

        // Execute inline (no asynq client available in this service)
        let email_logic =
            super::email::SendEmailLogic::new(self.repos.clone(), self.config.clone());
        if let Err(e) = email_logic.execute(&raw).await {
            tracing::error!(
                "[CheckSubscription] send {kind} email for subscribe {subscribe_id}: {e}"
            );
        }
    }
}
