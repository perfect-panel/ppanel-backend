use std::sync::Arc;

use axum::routing::get;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::Layer as _;
use tracing_subscriber::EnvFilter;

pub mod adapter;
pub mod cache;
pub mod config;
pub mod db;
pub mod exchange_rate;
pub mod handler;
pub mod middleware;
pub mod migration;
pub mod model;
pub mod queue;
pub mod repository;
pub mod scheduler;
pub mod service;
pub mod tracing_otel;

async fn health() -> &'static str {
    "ok"
}

/// Initialise the tracing subscriber from `LogConfig`.
///
/// Mirrors the Go `LogConf` initialisation logic:
///   - mode "console" (or empty / unrecognised) → stdout
///   - mode "file"   → daily-rotating file in `path/`
///   - mode "volume" → daily-rotating file in `path/{service_name}/{hostname}/`
/// encoding "json" uses JSON format; anything else uses the default pretty format.
///
/// If `otel` is true the OpenTelemetry bridge layer is added so every tracing
/// span is also exported through the global OTel provider (set up by
/// `tracing_otel::init_otel` before this call).
fn init_tracing(cfg: &config::LogConfig, otel: bool) {
    let filter = EnvFilter::builder()
        .parse_lossy(format!("ppanel_backend={}", cfg.level));

    match cfg.mode.as_str() {
        "file" | "volume" => {
            let dir = if cfg.mode == "volume" {
                format!("{}/{}/{}", cfg.path, cfg.service_name, hostname())
            } else {
                cfg.path.clone()
            };
            if let Err(e) = std::fs::create_dir_all(&dir) {
                eprintln!("failed to create log directory {dir}: {e}");
            }
            let file_appender = tracing_appender::rolling::daily(&dir, "app.log");
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            std::mem::forget(guard);

            if cfg.encoding == "json" {
                let sub = tracing_subscriber::registry()
                    .with(tracing_subscriber::fmt::layer()
                        .json()
                        .with_writer(non_blocking)
                        .with_filter(filter));
                if otel {
                    sub.with(tracing_opentelemetry::layer()).init();
                } else {
                    sub.init();
                }
            } else {
                let sub = tracing_subscriber::registry()
                    .with(tracing_subscriber::fmt::layer()
                        .with_writer(non_blocking)
                        .with_filter(filter));
                if otel {
                    sub.with(tracing_opentelemetry::layer()).init();
                } else {
                    sub.init();
                }
            }
        }
        _ => {
            // "console" or default
            if cfg.encoding == "json" {
                let sub = tracing_subscriber::registry()
                    .with(tracing_subscriber::fmt::layer()
                        .json()
                        .with_filter(filter));
                if otel {
                    sub.with(tracing_opentelemetry::layer()).init();
                } else {
                    sub.init();
                }
            } else {
                let sub = tracing_subscriber::registry()
                    .with(tracing_subscriber::fmt::layer()
                        .with_filter(filter));
                if otel {
                    sub.with(tracing_opentelemetry::layer()).init();
                } else {
                    sub.init();
                }
            }
        }
    }
}

/// Returns the machine hostname, falling back to `"unknown"`.
fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| {
            // Try reading /etc/hostname on Linux.
            std::fs::read_to_string("/etc/hostname").map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

#[tokio::main]
async fn main() {
    // ── Load configuration ──────────────────────────────────────────────
    let cfg = Arc::new(config::Config::load());

    // ── Initialise OpenTelemetry provider (before tracing subscriber) ───
    // Guard must stay alive for the process lifetime to flush pending spans.
    let _otel_guard = tracing_otel::init_otel(&cfg.trace);
    let has_otel = _otel_guard.is_some();

    // ── Initialise tracing subscriber from LogConfig ────────────────────
    init_tracing(&cfg.logger, has_otel);
    tracing::info!(host = %cfg.host, port = %cfg.port, "configuration loaded");

    // ── Initialise database ─────────────────────────────────────────────
    let db = db::init_pool(cfg.database_config())
        .await
        .expect("failed to connect to database");
    tracing::info!("database connected");

    // ── Ensure schema is present (invoke Go migrate tool if needed) ────
    migration::ensure_schema(&db, &cfg.database).await;

    // ── Seed initial admin account if needed ────────────────────────────
    migration::create_admin_user(&db, &cfg.administrator.email, &cfg.administrator.password)
        .await
        .expect("failed to create admin user");

    // ── Initialise Redis cache ──────────────────────────────────────────
    let cache = cache::Cache::new(&cfg.redis)
        .await
        .expect("failed to connect to redis");
    let cache = std::sync::Arc::new(cache);
    tracing::info!("redis connected");

    // ── Build queue client ──────────────────────────────────────────────
    let queue_client = queue::client::QueueClient::new(&queue::redis_url(&cfg.redis))
        .await
        .expect("failed to connect asynq queue client");
    tracing::info!("queue client connected");

    // ── Build repositories & router ─────────────────────────────────────
    let repos = std::sync::Arc::new(repository::Repositories::new(db));
    let queue_repos = Arc::clone(&repos);
    let state = handler::AppState {
        repos,
        config: cfg.clone(),
        cache,
        queue: queue_client,
    };
    let app = handler::routes::register_routes(state).route("/health", get(health));

    // ── Start background services ───────────────────────────────────────
    let _scheduler = scheduler::Service::start(&cfg)
        .await
        .expect("failed to start scheduler");

    let mut consumer = queue::Service::new(&cfg, queue_repos)
        .await
        .expect("failed to start queue consumer");

    let addr = format!("{}:{}", cfg.host, cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("failed to bind {addr}: {e}"));
    tracing::info!(
        "listening on {}",
        listener.local_addr().expect("listener bound")
    );

    // Block on HTTP server; on shutdown also stop the consumer.
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            consumer.shutdown().await.unwrap_or_else(|e| {
                tracing::error!("queue consumer shutdown error: {e}");
            });
        })
        .await
        .unwrap_or_else(|e| panic!("server error: {e}"));

    // scheduler stops automatically via Drop when `_scheduler` goes out of scope
}
