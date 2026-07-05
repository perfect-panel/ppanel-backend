use anyhow::Context;

use crate::model::dto::auth::{AuthMethodConfig, GetAuthMethodConfigRequest};
use crate::repository::auth::AuthRepo;

pub async fn get_auth_method_config(
    repo: &dyn AuthRepo,
    req: GetAuthMethodConfigRequest,
) -> anyhow::Result<AuthMethodConfig> {
    let m = repo
        .find_one_by_method(&req.method)
        .await
        .context("find auth method by method")?;
    Ok(AuthMethodConfig {
        id: m.id,
        method: m.method,
        config: serde_json::from_str(&m.config).unwrap_or_default(),
        enabled: m.enabled.unwrap_or(false),
    })
}
