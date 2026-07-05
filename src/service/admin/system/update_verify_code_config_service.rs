use crate::model::dto::VerifyCodeConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "verify_code";

/// Persist verify-code configuration to the `system` table.
pub async fn update_verify_code_config(
    repos: &Repositories,
    req: VerifyCodeConfig,
) -> Result<(), anyhow::Error> {
    persist_config(
        repos,
        CATEGORY,
        "verify_code_expire_time",
        &req.verify_code_expire_time.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "verify_code_limit",
        &req.verify_code_limit.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "verify_code_interval",
        &req.verify_code_interval.to_string(),
    )
    .await?;
    Ok(())
}
