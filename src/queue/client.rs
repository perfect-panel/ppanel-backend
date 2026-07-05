//! Shared asynq queue client.
//!
//! Wraps `asynq::client::Client` in an `Arc` so it can be cheaply cloned
//! into `AppState` and shared across every handler.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use serde::Serialize;

use asynq::backend::RedisConnectionType;

/// Thin, cheaply-cloneable wrapper around the asynq client.
#[derive(Clone)]
pub struct QueueClient {
    inner: Arc<asynq::client::Client>,
}

impl QueueClient {
    /// Connect to Redis and build a new [`QueueClient`].
    pub async fn new(redis_url: &str) -> anyhow::Result<Self> {
        let redis_cfg =
            RedisConnectionType::single(redis_url).context("build redis connection for queue")?;
        let client = asynq::client::Client::new(redis_cfg)
            .await
            .context("connect asynq queue client")?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Enqueue a task for immediate processing.
    pub async fn enqueue(&self, task_type: &str, payload: &[u8]) -> anyhow::Result<()> {
        let task =
            asynq::task::Task::new(task_type, payload).context("build asynq task")?;
        self.inner
            .enqueue(task)
            .await
            .context("enqueue task")?;
        Ok(())
    }

    /// Enqueue a task with a JSON-serialisable payload for immediate processing.
    pub async fn enqueue_json<T: Serialize>(
        &self,
        task_type: &str,
        payload: &T,
    ) -> anyhow::Result<()> {
        let bytes = serde_json::to_vec(payload).context("serialize task payload")?;
        self.enqueue(task_type, &bytes).await
    }

    /// Enqueue a task to be processed after `delay`.
    pub async fn enqueue_delayed(
        &self,
        task_type: &str,
        payload: &[u8],
        delay: Duration,
    ) -> anyhow::Result<()> {
        let task =
            asynq::task::Task::new(task_type, payload).context("build asynq task")?;
        self.inner
            .enqueue_in(task, delay)
            .await
            .context("enqueue delayed task")?;
        Ok(())
    }

    /// Enqueue a delayed task with a JSON-serialisable payload.
    pub async fn enqueue_json_delayed<T: Serialize>(
        &self,
        task_type: &str,
        payload: &T,
        delay: Duration,
    ) -> anyhow::Result<()> {
        let bytes = serde_json::to_vec(payload).context("serialize task payload")?;
        self.enqueue_delayed(task_type, &bytes, delay).await
    }
}
