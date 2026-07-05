//! Shared helpers for auth services.

use anyhow::anyhow;
use result::code_error::CodeError;
use result::error_code;

/// Verify a Cloudflare Turnstile token.
///
/// Returns `Err(TOO_MANY_REQUESTS)` when verification fails.
/// Call-site guards the `debug` flag — pass `enabled = config.verify.login_verify && config.model != "dev"`.
pub async fn check_turnstile(
    enabled: bool,
    secret: &str,
    token: &str,
    ip: &str,
) -> anyhow::Result<()> {
    if !enabled {
        return Ok(());
    }
    let ok = turnstile::verify(secret, token, ip).await.unwrap_or(false);
    if !ok {
        return Err(anyhow!(CodeError::new_err_code(error_code::TOO_MANY_REQUESTS)));
    }
    Ok(())
}
