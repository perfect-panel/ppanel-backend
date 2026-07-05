use crate::model::dto::SubscribeConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "subscribe";

/// Persist subscribe configuration to the `system` table.
pub async fn update_subscribe_config(
    repos: &Repositories,
    req: SubscribeConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "single_model", &req.single_model.to_string()).await?;
    persist_config(repos, CATEGORY, "subscribe_path", &req.subscribe_path).await?;
    persist_config(repos, CATEGORY, "subscribe_domain", &req.subscribe_domain).await?;
    persist_config(repos, CATEGORY, "pan_domain", &req.pan_domain.to_string()).await?;
    persist_config(
        repos,
        CATEGORY,
        "user_agent_limit",
        &req.user_agent_limit.to_string(),
    )
    .await?;
    persist_config(repos, CATEGORY, "user_agent_list", &req.user_agent_list).await?;
    persist_config(repos, CATEGORY, "show_tutorial", &req.show_tutorial.to_string()).await?;
    Ok(())
}
