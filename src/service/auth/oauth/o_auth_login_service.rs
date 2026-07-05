//! OAuth login — generates provider redirect URLs.
//!
//! Mirrors Go `oAuthLoginLogic.go`:
//!   - Loads provider config from `auth_method` table
//!   - Generates a random state code, stores it in Redis (5 min TTL)
//!   - Builds and returns the provider's authorization URL

use std::sync::Arc;

use anyhow::anyhow;

use crate::cache::Cache;
use crate::config::Config;
use crate::model::dto::auth::{OAthLoginRequest, OAuthLoginResponse};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

const STATE_TTL_SECS: i64 = 300; // 5 minutes

pub struct OAuthLoginService {
    repos: Arc<Repositories>,
    _config: Arc<Config>,
    cache: Arc<Cache>,
}

impl OAuthLoginService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self { repos, _config: config, cache }
    }

    pub async fn login(&self, req: OAthLoginRequest) -> Result<OAuthLoginResponse, anyhow::Error> {
        let redirect = match req.method.as_str() {
            "google"   => self.google(&req).await?,
            "apple"    => self.apple(&req).await?,
            "telegram" => self.telegram(&req).await?,
            other => return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::GET_AUTHENTICATOR_ERROR,
                format!("unsupported oauth method: {other}"),
            ))),
        };
        Ok(OAuthLoginResponse { redirect })
    }

    // ── Google ────────────────────────────────────────────────────────────

    async fn google(&self, req: &OAthLoginRequest) -> Result<String, anyhow::Error> {
        let method = self.repos.auth.find_one_by_method("google").await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let cfg: oauth::GoogleConfig = serde_json::from_str(&method.config.to_string())
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        // Generate state + PKCE verifier; store state->redirect_url in Redis
        let state = random_state();
        let redis_key = format!("google:{}", state);
        self.cache.set_ex(&redis_key, &req.redirect, STATE_TTL_SECS).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        // arctic-oauth: Google.authorization_url(state, scopes, code_verifier)
        let code_verifier = oauth::generate_code_verifier();
        let google = oauth::Google::new(&cfg.client_id, &cfg.client_secret, &req.redirect);
        let url = google.authorization_url(&state, &["openid", "email", "profile"], &code_verifier);
        Ok(url.to_string())
    }

    // ── Apple ─────────────────────────────────────────────────────────────

    async fn apple(&self, req: &OAthLoginRequest) -> Result<String, anyhow::Error> {
        let method = self.repos.auth.find_one_by_method("apple").await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let cfg: oauth::AppleConfig = serde_json::from_str(&method.config.to_string())
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let state = random_state();
        // Go uses "telegram:<state>" key for Apple state — preserve that behaviour
        let redis_key = format!("telegram:{}", state);
        self.cache.set_ex(&redis_key, &req.redirect, STATE_TTL_SECS).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let callback_url = format!("{}/v1/auth/oauth/callback/apple", cfg.redirect_url);
        // Arctic Apple expects PKCS#8 DER; `client_secret` in our config is the raw p8 PEM.
        // Build the URL manually to match Go behaviour (no Arctic Apple client for redirect).
        let url = format!(
            "https://appleid.apple.com/auth/authorize?client_id={}&redirect_uri={}&response_type=code&state={}&scope=name%20email&response_mode=form_post",
            cfg.client_id,
            urlencoding::encode(&callback_url),
            state,
        );
        Ok(url)
    }

    // ── Telegram ──────────────────────────────────────────────────────────

    async fn telegram(&self, req: &OAthLoginRequest) -> Result<String, anyhow::Error> {
        let method = self.repos.auth.find_one_by_method("telegram").await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let cfg: oauth::TelegramConfig = serde_json::from_str(&method.config.to_string())
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let state = random_state();
        // Go uses "apple:<state>" key for Telegram state — preserve that behaviour
        let redis_key = format!("apple:{}", state);
        self.cache.set_ex(&redis_key, &req.redirect, STATE_TTL_SECS).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let url = generate_telegram_oauth_url(&cfg.bot_token, &state, &req.redirect);
        Ok(url)
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// Generate a random 8-character alphanumeric state token (mirrors Go `random.KeyNew`).
fn random_state() -> String {
    use rand::Rng;
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..8).map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char).collect()
}

/// Build a Telegram login widget redirect URL.
/// Mirrors Go `telegram.GenerateTelegramOAuthURL`.
fn generate_telegram_oauth_url(bot_token: &str, state: &str, redirect: &str) -> String {
    let bot_id = bot_token.split(':').next().unwrap_or("");
    format!(
        "https://oauth.telegram.org/auth?bot_id={}&origin={}&embed=0&request_access=write&return_to={}",
        bot_id,
        urlencoding::encode(redirect),
        urlencoding::encode(&format!("{}?state={}", redirect, state)),
    )
}
