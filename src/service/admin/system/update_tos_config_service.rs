use crate::model::dto::TosConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "tos";

/// Persist Terms-of-Service configuration to the `system` table.
pub async fn update_tos_config(
    repos: &Repositories,
    req: TosConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "tos_content", &req.tos_content).await?;
    Ok(())
}
