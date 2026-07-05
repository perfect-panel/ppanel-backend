use std::sync::Arc;

use serde::Deserialize;

use crate::config::Config;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

// ─── SendEmail payload ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct SendEmailPayload {
    #[serde(rename = "type", default)]
    pub type_: i16,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(default)]
    pub subject: String,
    /// Raw JSON value — content map forwarded to templates.
    #[serde(default)]
    pub content: serde_json::Value,
}

// Email type constants (matches Go `queue/types`)
const EMAIL_TYPE_VERIFY: i16 = 1;
const EMAIL_TYPE_MAINTENANCE: i16 = 2;
const EMAIL_TYPE_EXPIRATION: i16 = 3;
const EMAIL_TYPE_TRAFFIC_EXCEED: i16 = 4;
const EMAIL_TYPE_CUSTOM: i16 = 5;

/// Port of `server/queue/logic/email/sendEmailLogic.go`.
pub struct SendEmailLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl SendEmailLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self, raw: &[u8]) -> anyhow::Result<()> {
        let payload: SendEmailPayload = match serde_json::from_slice(raw) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("[SendEmailLogic] deserialise payload: {e}");
                return Ok(());
            }
        };

        // Build sender
        let sender = match email::new_sender(
            &self.config.email.platform,
            &self.config.email.platform_config,
            &self.config.site.site_name,
        ) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("[SendEmailLogic] new_sender: {e}");
                return Ok(());
            }
        };

        // Render body from template based on email type
        let body = match self.render_body(&payload) {
            Some(b) => b,
            None => return Ok(()),
        };

        let status: i16 = match sender.send(&[payload.email.clone()], &payload.subject, &body).await {
            Ok(()) => 1,
            Err(e) => {
                tracing::error!("[SendEmailLogic] send failed to {}: {e}", payload.email);
                2
            }
        };

        Telemetry::email_message(
            &self.repos,
            0,
            &payload.email,
            Some(payload.subject.clone()),
            payload.content.clone(),
            &self.config.email.platform,
            "",
            status,
        )
        .await;

        Ok(())
    }

    fn render_body(&self, payload: &SendEmailPayload) -> Option<String> {
        let cfg = &self.config.email;
        let tpl_src = match payload.type_ {
            EMAIL_TYPE_VERIFY => cfg.verify_email_template.as_str(),
            EMAIL_TYPE_MAINTENANCE => cfg.maintenance_email_template.as_str(),
            EMAIL_TYPE_EXPIRATION => cfg.expiration_email_template.as_str(),
            EMAIL_TYPE_TRAFFIC_EXCEED => cfg.traffic_exceed_email_template.as_str(),
            EMAIL_TYPE_CUSTOM => {
                // For custom type use the "content" field directly as HTML
                if let Some(s) = payload.content.get("content").and_then(|v| v.as_str()) {
                    return Some(s.to_string());
                }
                tracing::error!("[SendEmailLogic] custom email missing content string");
                return None;
            }
            other => {
                tracing::error!("[SendEmailLogic] unknown email type {other}");
                return None;
            }
        };

        let ctx = json_to_gtmpl(payload.content.clone());
        match gtmpl::template(tpl_src, ctx) {
            Ok(rendered) => Some(rendered),
            Err(e) => {
                tracing::error!("[SendEmailLogic] template render (type={}): {e}", payload.type_);
                None
            }
        }
    }
}

// ─── BatchEmail ──────────────────────────────────────────────────────────────

/// Port of `server/queue/logic/email/batchEmailLogic.go`.
pub struct BatchEmailLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl BatchEmailLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self, raw: &[u8]) -> anyhow::Result<()> {
        if raw.is_empty() {
            tracing::error!("[BatchEmailLogic] empty payload");
            return Ok(());
        }

        let task_id: i64 = match std::str::from_utf8(raw)
            .ok()
            .and_then(|s| s.trim().parse().ok())
        {
            Some(id) => id,
            None => {
                tracing::error!("[BatchEmailLogic] invalid task ID in payload");
                return Ok(());
            }
        };

        let task_info = match self.repos.task.find_one(task_id).await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!("[BatchEmailLogic] find_one({task_id}): {e}");
                return Ok(());
            }
        };

        if task_info.status != 0 {
            tracing::info!("[BatchEmailLogic] task {task_id} already processed (status={})", task_info.status);
            return Ok(());
        }

        let sender = match email::new_sender(
            &self.config.email.platform,
            &self.config.email.platform_config,
            &self.config.site.site_name,
        ) {
            Ok(s) => std::sync::Arc::from(s),
            Err(e) => {
                tracing::error!("[BatchEmailLogic] new_sender: {e}");
                return Ok(());
            }
        };

        // Use the global WorkerManager (created at startup) or create a local one
        if let Some(mgr) = email::get_global_manager() {
            mgr.add_worker(task_id).await;
        } else {
            // Fallback: create a transient manager backed by the task repo adapter
            let repo_adapter = Arc::new(TaskRepoAdapter {
                inner: self.repos.task.as_ref() as *const _,
            });
            // SAFETY: We hold `self.repos` for the lifetime of this call.
            // The adapter is only used within this async scope.
            let mgr = email::WorkerManager::new(repo_adapter, sender);
            mgr.add_worker(task_id).await;
        }

        Ok(())
    }
}

// ─── TaskRepo adapter bridging `email::manager::TaskRepo` → our repo ─────────

use std::sync::Mutex;

struct TaskRepoAdapter {
    // raw pointer: only safe because the adapter is used within a single
    // async scope where `repos.task` is guaranteed alive.
    inner: *const dyn crate::repository::task::TaskRepo,
}

unsafe impl Send for TaskRepoAdapter {}
unsafe impl Sync for TaskRepoAdapter {}

#[async_trait::async_trait]
impl email::manager::TaskRepo for TaskRepoAdapter {
    async fn find_one(&self, id: i64) -> Result<email::worker::TaskInfo, anyhow::Error> {
        // SAFETY: pointer is valid for the duration of the call (see above).
        let repo = unsafe { &*self.inner };
        let t = repo.find_one(id).await?;
        Ok(email::worker::TaskInfo {
            id: t.id,
            type_: t.type_,
            scope: t.scope.clone().unwrap_or_default(),
            content: t.content.clone().unwrap_or_default(),
            status: t.status,
            errors: t.errors.clone().unwrap_or_default(),
            total: t.total,
            current: t.current,
        })
    }

    async fn update(&self, data: &email::worker::TaskInfo) -> Result<(), anyhow::Error> {
        let repo = unsafe { &*self.inner };
        let mut t = repo.find_one(data.id).await?;
        t.status = data.status;
        t.current = data.current;
        t.errors = if data.errors.is_empty() { None } else { Some(data.errors.clone()) };
        repo.update(&t).await?;
        Ok(())
    }

    async fn update_status(&self, id: i64, status: i16) -> Result<(), anyhow::Error> {
        let repo = unsafe { &*self.inner };
        repo.update_status(id, status).await?;
        Ok(())
    }

    fn is_cancelled(&self, _id: i64) -> bool {
        false
    }
}

fn json_to_gtmpl(v: serde_json::Value) -> gtmpl::Value {
    use std::collections::HashMap;
    match v {
        serde_json::Value::Null => gtmpl::Value::Nil,
        serde_json::Value::Bool(b) => gtmpl::Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                gtmpl::Value::Number(gtmpl_value::Number::from(i))
            } else if let Some(f) = n.as_f64() {
                gtmpl::Value::Number(gtmpl_value::Number::from(f))
            } else {
                gtmpl::Value::Number(gtmpl_value::Number::from(0_i64))
            }
        }
        serde_json::Value::String(s) => gtmpl::Value::String(s),
        serde_json::Value::Array(arr) => {
            gtmpl::Value::Array(arr.into_iter().map(json_to_gtmpl).collect())
        }
        serde_json::Value::Object(map) => {
            let m: HashMap<String, gtmpl::Value> =
                map.into_iter().map(|(k, v)| (k, json_to_gtmpl(v))).collect();
            gtmpl::Value::Map(m)
        }
    }
}
