use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::PrivacyPolicyConfig;
use crate::repository::Repositories;

/// Read privacy-policy configuration.
///
/// Stored in the same `tos` category as Terms of Service (matches Go).
pub async fn get_privacy_policy_config(
    repos: &Repositories,
) -> Result<PrivacyPolicyConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_tos_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = PrivacyPolicyConfig {
        privacy_policy: String::new(),
    };
    for row in rows {
        if row.key == "privacy_policy" {
            resp.privacy_policy = row.value;
        }
    }
    Ok(resp)
}
