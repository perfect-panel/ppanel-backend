use crate::model::dto::SiteConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "site";

/// Persist site configuration to the `system` table.
pub async fn update_site_config(
    repos: &Repositories,
    req: SiteConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "host", &req.host).await?;
    persist_config(repos, CATEGORY, "site_name", &req.site_name).await?;
    persist_config(repos, CATEGORY, "site_desc", &req.site_desc).await?;
    persist_config(repos, CATEGORY, "site_logo", &req.site_logo).await?;
    persist_config(repos, CATEGORY, "keywords", &req.keywords).await?;
    persist_config(repos, CATEGORY, "custom_html", &req.custom_html).await?;
    persist_config(repos, CATEGORY, "custom_data", &req.custom_data).await?;
    Ok(())
}
