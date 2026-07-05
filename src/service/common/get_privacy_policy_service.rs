use crate::model::dto::system::PrivacyPolicyConfig;
use crate::repository::Repositories;
use anyhow::anyhow;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_privacy_policy(repos: &Repositories) -> anyhow::Result<PrivacyPolicyConfig> {
    let configs = repos
        .system
        .get_tos_config()
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let privacy_policy = configs
        .iter()
        .find(|s| s.key == "PrivacyPolicy")
        .map(|s| s.value.clone())
        .unwrap_or_default();

    Ok(PrivacyPolicyConfig { privacy_policy })
}
