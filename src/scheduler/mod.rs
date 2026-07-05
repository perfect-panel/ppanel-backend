use std::sync::Arc;
use std::time::Duration;

use asynq::backend::RedisConnectionType;
use asynq::client::Client;
use asynq::scheduler::{PeriodicTask, Scheduler};

use crate::config;
use crate::queue::types;

pub struct Service {
    _scheduler: Arc<Scheduler>,
}

impl Service {
    /// Build a Redis URL from our config (compatible with `redis://` scheme).
    fn redis_url(cfg: &config::RedisConfig) -> String {
        let db = cfg.db;
        if cfg.pass.is_empty() {
            format!("redis://{}/{}", cfg.host, db)
        } else {
            format!("redis://:{}@{}/{}", cfg.pass, cfg.host, db)
        }
    }

    /// Create and start the scheduler.
    ///
    /// Registers all periodic tasks (mirrors `scheduler/scheduler.go:Start()`).
    pub async fn start(cfg: &config::Config) -> anyhow::Result<Self> {
        let redis_cfg = RedisConnectionType::single(Self::redis_url(&cfg.redis))?;
        let client = Arc::new(Client::new(redis_cfg).await?);

        let scheduler = Arc::new(Scheduler::new(client, Some(Duration::from_secs(30))).await?);

        // ── register periodic tasks ──────────────────────────────────────

        // every 60s: check subscription
        Self::register(
            &scheduler,
            types::SCHEDULER_CHECK_SUBSCRIPTION,
            "@every 60s",
            "default",
        )
        .await?;

        // every day at 00:30: reset traffic
        Self::register(
            &scheduler,
            types::SCHEDULER_RESET_TRAFFIC,
            "30 0 * * *",
            "default",
        )
        .await?;

        // every day at 00:00: traffic stat
        Self::register(
            &scheduler,
            types::SCHEDULER_TRAFFIC_STAT,
            "0 0 * * *",
            "default",
        )
        .await?;

        // every day at 01:00: quota task
        Self::register(
            &scheduler,
            types::FORTHWITH_QUOTA_TASK,
            "0 1 * * *",
            "default",
        )
        .await?;

        tracing::info!("scheduler started with 4 periodic tasks");

        Ok(Self { _scheduler: scheduler })
    }

    async fn register(
        scheduler: &Scheduler,
        task_type: &str,
        cron: &str,
        queue: &str,
    ) -> anyhow::Result<()> {
        let task = PeriodicTask::new(
            task_type.to_string(),
            cron.to_string(),
            Vec::new(),
            queue.to_string(),
        )?;
        scheduler.register(task, queue).await?;
        tracing::info!("registered periodic task: {task_type} ({cron})");
        Ok(())
    }
}
