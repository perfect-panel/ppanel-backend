use crate::model::dto::RegisterConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "register";

/// Persist registration configuration to the `system` table.
pub async fn update_register_config(
    repos: &Repositories,
    req: RegisterConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "stop_register", &req.stop_register.to_string()).await?;
    persist_config(repos, CATEGORY, "enable_trial", &req.enable_trial.to_string()).await?;
    persist_config(
        repos,
        CATEGORY,
        "trial_subscribe",
        &req.trial_subscribe.to_string(),
    )
    .await?;
    persist_config(repos, CATEGORY, "trial_time", &req.trial_time.to_string()).await?;
    persist_config(repos, CATEGORY, "trial_time_unit", &req.trial_time_unit).await?;
    persist_config(
        repos,
        CATEGORY,
        "enable_ip_register_limit",
        &req.enable_ip_register_limit.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "ip_register_limit",
        &req.ip_register_limit.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "ip_register_limit_duration",
        &req.ip_register_limit_duration.to_string(),
    )
    .await?;
    Ok(())
}
