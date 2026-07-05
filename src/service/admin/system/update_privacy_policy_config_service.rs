use crate::model::dto::PrivacyPolicyConfig;
use crate::repository::Repositories;
use crate::service::admin::system::update_config::persist_config;

const CATEGORY: &str = "tos";

/// Persist privacy-policy configuration. Stored alongside the TOS entry
/// (matches the Go `updatePrivacyPolicyConfigLogic` behaviour).
pub async fn update_privacy_policy_config(
    repos: &Repositories,
    req: PrivacyPolicyConfig,
) -> Result<(), anyhow::Error> {
    persist_config(repos, CATEGORY, "privacy_policy", &req.privacy_policy).await?;
    Ok(())
}
