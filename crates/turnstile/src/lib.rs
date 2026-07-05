//! Cloudflare Turnstile token verification.
//! Port of `server/pkg/turnstile`.

use serde::{Deserialize, Serialize};

const VERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

#[derive(Debug, Serialize)]
struct VerifyRequest<'a> {
    secret: &'a str,
    response: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    remoteip: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    idempotency_key: &'a str,
}

#[derive(Debug, Deserialize)]
struct VerifyResponse {
    success: bool,
    #[serde(rename = "error-codes", default)]
    error_codes: Vec<String>,
}

/// Verify a Turnstile challenge token.
///
/// - `secret` — site secret key from Cloudflare dashboard
/// - `token`  — value of `cf-turnstile-response` submitted by the browser
/// - `ip`     — optional client IP (pass `""` to omit)
pub async fn verify(secret: &str, token: &str, ip: &str) -> anyhow::Result<bool> {
    verify_with_key(secret, token, ip, "").await
}

/// Verify with idempotency key (prevents the same token being accepted twice).
pub async fn verify_with_key(
    secret: &str,
    token: &str,
    ip: &str,
    idempotency_key: &str,
) -> anyhow::Result<bool> {
    let client = reqwest::Client::new();
    let req = VerifyRequest { secret, response: token, remoteip: ip, idempotency_key };
    let resp: VerifyResponse = client
        .post(VERIFY_URL)
        .json(&req)
        .send()
        .await?
        .json()
        .await?;

    if !resp.success && !resp.error_codes.is_empty() {
        tracing::warn!(codes = ?resp.error_codes, "turnstile verification failed");
    }
    Ok(resp.success)
}

/// Generate a random UUID suitable for use as an idempotency key.
pub fn random_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
