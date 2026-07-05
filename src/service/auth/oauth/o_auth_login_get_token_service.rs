//! OAuth token exchange — mirrors Go `oAuthLoginGetTokenLogic.go`.
//!
//! Flow:
//!   1. Route to provider handler (google / apple / telegram)
//!   2. Validate state from Redis, exchange code for tokens
//!   3. Extract user info (open_id, email, avatar)
//!   4. findOrRegisterUser in DB
//!   5. Issue JWT + Redis session
//!   6. Record login / register audit logs via Telemetry

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;
use uuid::Uuid;

use crate::cache::Cache;
use crate::config::cache_key::SESSION_ID_KEY;
use crate::config::Config;
use crate::model::dto::auth::{LoginResponse, OAuthLoginGetTokenRequest};
use crate::model::entity::user::{AuthMethods, User, UserSubscribe};
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;
use result::code_error::CodeError;
use result::error_code;

const TELEGRAM_DOMAIN: &str = "ppanel.com";
const AUTH_EXPIRE_SECS: i64 = 86400;

pub struct OAuthLoginGetTokenService {
    repos: Arc<Repositories>,
    config: Arc<Config>,
    cache: Arc<Cache>,
}

impl OAuthLoginGetTokenService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self { repos, config, cache }
    }

    pub async fn get_token(
        &self,
        req: OAuthLoginGetTokenRequest,
        ip: &str,
        user_agent: &str,
    ) -> Result<LoginResponse, anyhow::Error> {
        let (auth_type, open_id, email, avatar) = match req.method.as_str() {
            "google"   => self.handle_google(&req).await?,
            "apple"    => self.handle_apple(&req).await?,
            "telegram" => self.handle_telegram(&req).await?,
            other => return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::GET_AUTHENTICATOR_ERROR,
                format!("unsupported oauth method: {other}"),
            ))),
        };

        let user = self.find_or_register_user(
            &auth_type, &open_id, email.as_deref(), avatar.as_deref(), ip, user_agent,
        ).await?;

        let token = self.issue_token(user.id).await?;

        Telemetry::login(&self.repos, user.id, &auth_type, ip, user_agent, true).await;

        Ok(LoginResponse { token })
    }

    // ── Google ────────────────────────────────────────────────────────────

    async fn handle_google(
        &self,
        req: &OAuthLoginGetTokenRequest,
    ) -> Result<(String, String, Option<String>, Option<String>), anyhow::Error> {
        let callback = req.callback.as_object()
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::INVALID_PARAMS)))?;

        let code = callback.get("code").and_then(|v| v.as_str()).unwrap_or("");
        let state = callback.get("state").and_then(|v| v.as_str()).unwrap_or("");

        let redirect = self.validate_state("google", state).await?;

        let method = self.repos.auth.find_one_by_method("google").await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let cfg: oauth::GoogleConfig = serde_json::from_str(&method.config.to_string())
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        // Exchange code for tokens — arctic-oauth Google requires a code_verifier.
        // In our stateless server flow the verifier is not stored, so we use an empty
        // verifier string (Google still accepts it when PKCE was not enforced at auth time).
        let google = oauth::Google::new(&cfg.client_id, &cfg.client_secret, &redirect);
        let tokens = google.validate_authorization_code(code, "").await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let info = oauth::OAuthUserInfo::from_google(&tokens)
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        Ok(("google".into(), info.open_id, info.email, info.picture))
    }

    // ── Apple ─────────────────────────────────────────────────────────────

    async fn handle_apple(
        &self,
        req: &OAuthLoginGetTokenRequest,
    ) -> Result<(String, String, Option<String>, Option<String>), anyhow::Error> {
        let callback = req.callback.as_object()
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::INVALID_PARAMS)))?;

        let code  = callback.get("code").and_then(|v| v.as_str()).unwrap_or("");
        let state = callback.get("state").and_then(|v| v.as_str()).unwrap_or("");

        // Apple state is stored under "telegram:<state>" key (matches Go behaviour)
        let _redirect = self.validate_state("telegram", state).await?;

        let method = self.repos.auth.find_one_by_method("apple").await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let cfg: oauth::AppleConfig = serde_json::from_str(&method.config.to_string())
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        // Arctic Apple requires a PKCS#8 DER private key; `client_secret` holds the PEM
        let pkcs8_der = pem_to_der(&cfg.client_secret)
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e)))?;

        let apple = oauth::Apple::new(
            &cfg.client_id, &cfg.team_id, &cfg.key_id, &pkcs8_der, &cfg.redirect_url,
        ).map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let tokens = apple.validate_authorization_code(code).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let info = oauth::OAuthUserInfo::from_apple(&tokens)
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        Ok(("apple".into(), info.open_id, info.email, info.picture))
    }

    // ── Telegram ──────────────────────────────────────────────────────────

    async fn handle_telegram(
        &self,
        req: &OAuthLoginGetTokenRequest,
    ) -> Result<(String, String, Option<String>, Option<String>), anyhow::Error> {
        let callback = req.callback.as_object()
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::INVALID_PARAMS)))?;

        let tg_auth_result = callback
            .get("tgAuthResult")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let method = self.repos.auth.find_one_by_method("telegram").await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let cfg: oauth::TelegramConfig = serde_json::from_str(&method.config.to_string())
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        let auth_data = oauth::parse_base64_and_validate(
            tg_auth_result,
            cfg.bot_token.as_bytes(),
        ).map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        // Check 24-hour expiry (mirrors Go AuthExpire = 86400)
        let now = Utc::now().timestamp();
        if now - auth_data.auth_date > AUTH_EXPIRE_SECS {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::ERROR, "telegram auth date expired",
            )));
        }

        let info = oauth::OAuthUserInfo::from_telegram(&auth_data);
        let email = Some(format!("{}@{}", auth_data.id, TELEGRAM_DOMAIN));
        let avatar = info.picture.clone();

        Ok(("telegram".into(), info.open_id, email, avatar))
    }

    // ── find or register user ─────────────────────────────────────────────

    async fn find_or_register_user(
        &self,
        auth_type: &str,
        open_id: &str,
        email: Option<&str>,
        avatar: Option<&str>,
        ip: &str,
        user_agent: &str,
    ) -> Result<User, anyhow::Error> {
        // Try to find existing auth method
        if let Some(am) = self.repos.user.find_auth_method_by_open_id(auth_type, open_id).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?
        {
            let user = self.repos.user.find_one_user(am.user_id).await
                .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;
            return Ok(user);
        }

        // Not found — register
        self.register_user(auth_type, open_id, email, avatar, ip, user_agent).await
    }

    async fn register_user(
        &self,
        auth_type: &str,
        open_id: &str,
        email: Option<&str>,
        avatar: Option<&str>,
        ip: &str,
        user_agent: &str,
    ) -> Result<User, anyhow::Error> {
        if self.config.invite.forced_invite {
            return Err(anyhow!(CodeError::new_err_code(error_code::INVITE_CODE_ERROR)));
        }

        // Verify email not already taken
        if let Some(em) = email {
            if let Some(_existing) = self.repos.user.find_one_by_email(em).await
                .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?
            {
                return Err(anyhow!(CodeError::new_err_code(error_code::USER_EXIST)));
            }
        }

        let now = Utc::now().timestamp_millis();
        let mut user = User {
            id: 0,
            password: String::new(),
            algo: String::new(),
            salt: None,
            avatar: avatar.unwrap_or("").to_string(),
            balance: 0,
            refer_code: String::new(),
            referer_id: 0,
            commission: 0,
            referral_percentage: self.config.invite.referral_percentage as i16,
            only_first_purchase: self.config.invite.only_first_purchase,
            gift_amount: 0,
            enable: true,
            is_admin: false,
            enable_balance_notify: false,
            enable_login_notify: false,
            enable_subscribe_notify: false,
            enable_trade_notify: false,
            rules: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        };

        user = self.repos.user.insert_user(&user).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        let refer_code = format!("U{:X}", user.id);
        let mut update_user = user.clone();
        update_user.refer_code = refer_code;
        self.repos.user.update_user(&update_user).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_UPDATE_ERROR, e.to_string())))?;

        // Create primary auth method (e.g. "google", "apple", "telegram")
        self.repos.user.insert_auth_method(&AuthMethods {
            id: 0,
            user_id: user.id,
            auth_type: auth_type.to_string(),
            auth_identifier: open_id.to_string(),
            verified: true,
            created_at: now,
            updated_at: now,
        }).await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        // Also link email auth method when an email is available
        if let Some(em) = email {
            let _ = self.repos.user.insert_auth_method(&AuthMethods {
                id: 0,
                user_id: user.id,
                auth_type: "email".to_string(),
                auth_identifier: em.to_string(),
                verified: true,
                created_at: now,
                updated_at: now,
            }).await;
        }

        // Activate trial if configured
        if self.config.register.enable_trial {
            let trial = self.activate_trial(user.id).await;
            if let Ok(ref sub) = trial {
                super::trial_cache::clear_trial_subscribe_cache(&self.cache, sub);
            }
        }

        Telemetry::register(&self.repos, user.id, auth_type, open_id, ip, user_agent).await;

        Ok(user)
    }

    async fn activate_trial(&self, user_id: i64) -> Result<UserSubscribe, anyhow::Error> {
        let sub = self.repos.subscribe.find_one(self.config.register.trial_subscribe).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let now = Utc::now();
        let expire_time = add_time(
            &self.config.register.trial_time_unit,
            self.config.register.trial_time,
            now,
        );
        let token = format!("Trial-{}-{}", user_id, Uuid::new_v4());

        let user_sub = UserSubscribe {
            id: 0, user_id, order_id: 0, subscribe_id: sub.id,
            start_time: now.timestamp_millis(),
            expire_time: expire_time.timestamp_millis(),
            finished_at: None, traffic: sub.traffic, download: 0, upload: 0,
            token, uuid: Uuid::new_v4().to_string(), status: 1, note: String::new(),
            created_at: now.timestamp_millis(), updated_at: now.timestamp_millis(),
        };

        self.repos.user.insert_subscribe(&user_sub).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        Ok(user_sub)
    }

    // ── token issuance ────────────────────────────────────────────────────

    async fn issue_token(&self, user_id: i64) -> Result<String, anyhow::Error> {
        let session_id = Uuid::new_v4().to_string();
        let (claims, seconds) = jwt::Claims::new(user_id, session_id.clone(), String::new());
        let token = jwt::generate_token(&claims, &self.config.jwt_auth.access_secret)
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;
        let session_key = format!("{}:{}", SESSION_ID_KEY, session_id);
        self.cache.set_ex(&session_key, &user_id.to_string(), seconds).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;
        Ok(token)
    }

    // ── state validation ──────────────────────────────────────────────────

    async fn validate_state(&self, provider: &str, state: &str) -> Result<String, anyhow::Error> {
        let redis_key = format!("{}:{}", provider, state);
        self.cache.get(&redis_key).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?
            .ok_or_else(|| anyhow!(CodeError::new_err_code_msg(
                error_code::ERROR,
                format!("invalid or expired state for provider: {}", provider),
            )))
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn add_time(unit: &str, amount: i64, from: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    match unit {
        "hour"  => from + chrono::Duration::hours(amount),
        "day"   => from + chrono::Duration::days(amount),
        "week"  => from + chrono::Duration::weeks(amount),
        "month" => from.checked_add_months(chrono::Months::new(amount as u32)).unwrap_or(from),
        "year"  => from.checked_add_months(chrono::Months::new((amount * 12) as u32)).unwrap_or(from),
        _       => from + chrono::Duration::days(amount),
    }
}

/// Decode a PEM-encoded private key to DER bytes (strips header/footer/newlines).
fn pem_to_der(pem: &str) -> Result<Vec<u8>, String> {
    let body = pem
        .lines()
        .filter(|l| !l.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(body.as_bytes())
        .map_err(|e| format!("base64 decode error: {e}"))
}
