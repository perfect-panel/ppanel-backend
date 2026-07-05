use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::VerifyConfig;
use crate::repository::Repositories;

/// Read verify (Turnstile) configuration from the `system` table (category = "verify").
pub async fn get_verify_config(
    repos: &Repositories,
) -> Result<VerifyConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_verify_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = VerifyConfig {
        turnstile_site_key: String::new(),
        turnstile_secret: String::new(),
        enable_login_verify: false,
        enable_register_verify: false,
        enable_reset_password_verify: false,
    };
    for row in rows {
        match row.key.as_str() {
            "turnstile_site_key" => resp.turnstile_site_key = row.value,
            "turnstile_secret" => resp.turnstile_secret = row.value,
            "enable_login_verify" | "login_verify" => {
                resp.enable_login_verify = row.value == "true"
            }
            "enable_register_verify" | "register_verify" => {
                resp.enable_register_verify = row.value == "true"
            }
            "enable_reset_password_verify" | "reset_password_verify" => {
                resp.enable_reset_password_verify = row.value == "true"
            }
            _ => {}
        }
    }
    Ok(resp)
}
