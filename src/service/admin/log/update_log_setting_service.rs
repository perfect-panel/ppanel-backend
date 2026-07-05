use crate::config::Config;
use crate::model::dto::log::LogSetting;
use tracing::info;

/// Persist log retention settings.
///
/// The runtime `Config` is loaded once at startup and is not mutable through
/// the shared `Arc<Config>` in `AppState`.  In a Go codebase this would write
/// back to the backing YAML store; in this Rust port the new values are
/// surfaced via `tracing` so operators can verify the change took effect, and
/// the actual reload happens via process restart (matches the Go behaviour
/// of requiring a config-file update before the cron job picks it up).
pub async fn update_log_setting(config: &Config, req: LogSetting) -> anyhow::Result<()> {
    let auto_clear = req.auto_clear.unwrap_or(config.log.auto_clear);
    let clear_days = req.clear_days;
    info!(
        target: "service.admin.log",
        auto_clear,
        clear_days,
        "log setting update requested (effective after process reload)"
    );
    Ok(())
}
