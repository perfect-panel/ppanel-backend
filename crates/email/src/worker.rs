use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::sender::Sender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub error: String,
    pub email: String,
    pub time: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerStatus {
    Idle = 0,
    Running = 1,
    Completed = 2,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailScope {
    #[serde(rename = "type")]
    pub type_: i16,
    #[serde(default)]
    pub register_start_time: i64,
    #[serde(default)]
    pub register_end_time: i64,
    #[serde(default)]
    pub recipients: Vec<String>,
    #[serde(default)]
    pub additional: Vec<String>,
    #[serde(default)]
    pub scheduled: i64,
    #[serde(default)]
    pub interval: i16,
    #[serde(default)]
    pub limit: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailContent {
    pub subject: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub id: i64,
    pub type_: i16,
    pub scope: String,
    pub content: String,
    pub status: i16,
    pub errors: String,
    pub total: i64,
    pub current: i64,
}

pub struct Worker {
    id: i64,
    repo: Arc<dyn crate::manager::TaskRepo>,
    sender: Arc<dyn Sender>,
    status: Arc<Mutex<WorkerStatus>>,
}

impl Worker {
    pub fn new(
        id: i64,
        repo: Arc<dyn crate::manager::TaskRepo>,
        sender: Arc<dyn Sender>,
    ) -> Self {
        Worker {
            id,
            repo,
            sender,
            status: Arc::new(Mutex::new(WorkerStatus::Idle)),
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub async fn is_running(&self) -> WorkerStatus {
        *self.status.lock().await
    }

    pub async fn start(&self) {
        let task_info = match self.repo.find_one(self.id).await {
            Ok(t) => t,
            Err(e) => {
                tracing::error!(
                    "Batch Send Email: Failed to find task, task_id={}, error={}",
                    self.id,
                    e
                );
                return;
            }
        };

        if task_info.status != 0 {
            tracing::error!(
                "Batch Send Email: Task already completed or in progress, task_id={}",
                self.id
            );
            return;
        }

        let scope: EmailScope = match serde_json::from_str(&task_info.scope) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!(
                    "Batch Send Email: Failed to parse task scope, task_id={}, error={}",
                    self.id,
                    e
                );
                return;
            }
        };

        if scope.recipients.is_empty() && scope.additional.is_empty() {
            tracing::error!(
                "Batch Send Email: No recipients or additional emails provided, task_id={}",
                self.id
            );
            return;
        }

        let content: EmailContent = match serde_json::from_str(&task_info.content) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!(
                    "Batch Send Email: Failed to parse task content, task_id={}, error={}",
                    self.id,
                    e
                );
                return;
            }
        };

        {
            let mut status = self.status.lock().await;
            *status = WorkerStatus::Running;
        }

        let mut recipients = scope.recipients.clone();
        recipients.extend(scope.additional.clone());
        remove_duplicates_and_empty(&mut recipients);

        if recipients.is_empty() {
            tracing::error!(
                "Batch Send Email: No valid recipients found, task_id={}",
                self.id
            );
            let mut status = self.status.lock().await;
            *status = WorkerStatus::Completed;
            return;
        }

        let interval = if scope.interval == 0 {
            Duration::from_secs(1)
        } else {
            Duration::from_secs(scope.interval as u64)
        };

        let mut errors: Vec<ErrorInfo> = Vec::new();
        let mut count: i64 = 0;

        for recipient in &recipients {
            if self.repo.is_cancelled(self.id) {
                tracing::info!(
                    "Batch Send Email: Worker stopped by cancellation, task_id={}",
                    self.id
                );
                return;
            }

            if task_info.status == 0 {
                // mark as in-progress via repo
                let _ = self.repo.update_status(self.id, 1).await;
            }

            if let Err(e) = self
                .sender
                .send(std::slice::from_ref(recipient), &content.subject, &content.content)
                .await
            {
                tracing::error!(
                    "Batch Send Email: Failed to send email, task_id={}, recipient={}, error={}",
                    self.id,
                    recipient,
                    e
                );
                errors.push(ErrorInfo {
                    error: e.to_string(),
                    email: recipient.clone(),
                    time: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64,
                });
            }

            count += 1;

            let mut updated = task_info.clone();
            updated.current = count;
            updated.errors = serde_json::to_string(&errors).unwrap_or_default();

            if let Err(e) = self.repo.update(&updated).await {
                tracing::error!(
                    "Batch Send Email: Failed to update task progress, task_id={}, error={}",
                    self.id,
                    e
                );
                let mut status = self.status.lock().await;
                *status = WorkerStatus::Completed;
            }

            sleep(interval).await;
        }

        let mut status = self.status.lock().await;
        *status = WorkerStatus::Completed;

        let mut finalized = task_info.clone();
        finalized.status = 2;
        finalized.current = count;
        finalized.errors = serde_json::to_string(&errors).unwrap_or_default();

        match self.repo.update(&finalized).await {
            Ok(_) => {
                tracing::info!(
                    "Batch Send Email: Task completed successfully, task_id={}, total_sent={}",
                    self.id,
                    count
                );
            }
            Err(e) => {
                tracing::error!(
                    "Batch Send Email: Failed to finalize task, task_id={}, error={}",
                    self.id,
                    e
                );
            }
        }
    }
}

fn remove_duplicates_and_empty(items: &mut Vec<String>) {
    let mut seen = std::collections::HashSet::new();
    items.retain(|item| {
        if item.is_empty() || seen.contains(item) {
            false
        } else {
            seen.insert(item.clone());
            true
        }
    });
}
