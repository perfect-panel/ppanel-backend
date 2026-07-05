use std::sync::Arc;

use asynq::backend::RedisConnectionType;
use asynq::config::ServerConfig;
use asynq::server::Server;

use crate::config;
use crate::config::Config;
use crate::repository::Repositories;

pub mod client;
pub mod handler;
pub mod service;
pub mod types;

pub fn redis_url(cfg: &config::RedisConfig) -> String {
    let db = cfg.db;
    if cfg.pass.is_empty() {
        format!("redis://{}/{}", cfg.host, db)
    } else {
        format!("redis://:{}@{}/{}", cfg.pass, cfg.host, db)
    }
}

pub struct Service {
    server: Server,
}

impl Service {
    pub async fn new(
        cfg: &Config,
        repos: Arc<Repositories>,
    ) -> anyhow::Result<Self> {
        let redis_cfg = RedisConnectionType::single(redis_url(&cfg.redis))?;
        let server_cfg = ServerConfig::new().concurrency(20);
        let mut server = Server::new(redis_cfg, server_cfg).await?;

        let config = Arc::new(cfg.clone());
        let mut mux = handler::register_all(repos, config);

        mux.handle_func("*", |task: asynq::task::Task| {
            tracing::warn!("unregistered task type: {}", task.get_type());
            Ok(())
        });

        server.start(mux).await?;
        tracing::info!("queue consumer started (concurrency=20)");

        Ok(Self { server })
    }

    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.server.shutdown().await?;
        Ok(())
    }
}
