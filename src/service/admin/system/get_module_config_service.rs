use std::env;

use crate::model::dto::ModuleConfig;

/// Build the module/service identity response from the runtime environment.
///
/// Mirrors Go `getModuleConfigLogic` — reads `SECRET_KEY` env var and uses
/// hard-coded service name + the crate version.
pub async fn get_module_config() -> Result<ModuleConfig, anyhow::Error> {
    let secret = env::var("SECRET_KEY").unwrap_or_default();
    Ok(ModuleConfig {
        secret,
        service_name: "PPanel".to_string(),
        service_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
