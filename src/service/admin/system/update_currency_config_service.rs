use crate::model::dto::CurrencyConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "currency";

/// Persist currency configuration to the `system` table.
pub async fn update_currency_config(
    repos: &Repositories,
    req: CurrencyConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "access_key", &req.access_key).await?;
    persist_config(repos, CATEGORY, "currency_unit", &req.currency_unit).await?;
    persist_config(repos, CATEGORY, "currency_symbol", &req.currency_symbol).await?;
    Ok(())
}
