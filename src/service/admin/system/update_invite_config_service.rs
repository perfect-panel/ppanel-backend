use crate::model::dto::InviteConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "invite";

/// Persist invite configuration to the `system` table.
pub async fn update_invite_config(
    repos: &Repositories,
    req: InviteConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "forced_invite", &req.forced_invite.to_string()).await?;
    persist_config(
        repos,
        CATEGORY,
        "referral_percentage",
        &req.referral_percentage.to_string(),
    )
    .await?;
    persist_config(
        repos,
        CATEGORY,
        "only_first_purchase",
        &req.only_first_purchase.to_string(),
    )
    .await?;
    Ok(())
}
