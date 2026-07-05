use crate::model::dto::VerifyConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "verify";

/// Persist verify (Turnstile) configuration to the `system` table.
pub async fn update_verify_config(
    repos: &Repositories,
    req: VerifyConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "turnstile_site_key", &req.turnstile_site_key).await?;
    persist_config(repos, CATEGORY, "turnstile_secret", &req.turnstile_secret).await?;
    persist_config(
        repos,
        CATEGORY,
        "enable_login_verify",
        &req.enable_login_verify.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "enable_register_verify",
        &req.enable_register_verify.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "enable_reset_password_verify",
        &req.enable_reset_password_verify.to_string(),
    )
    .await?;
    Ok(())
}
