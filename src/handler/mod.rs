pub mod admin;
pub mod auth;
pub mod common;
pub mod notify;
pub mod public;
pub mod routes;
pub mod server;
pub mod subscribe;
pub mod telegram;

use crate::cache::Cache;
use crate::config::Config;
use crate::queue::client::QueueClient;
use crate::repository::Repositories;

/// Shared application state passed to every handler via axum's `State`
/// extractor.
///
/// Carries the [`Repositories`] bundle, the loaded [`Config`], the
/// [`Cache`] client, and the [`QueueClient`] so handler code can reach
/// domain repos, runtime settings, Redis-backed cache, and the task queue
/// without global state.
#[derive(Clone)]
pub struct AppState {
    pub repos: std::sync::Arc<Repositories>,
    pub config: std::sync::Arc<Config>,
    pub cache: std::sync::Arc<Cache>,
    pub queue: QueueClient,
}
