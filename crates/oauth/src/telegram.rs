use base64::Engine;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use crate::error::OAuthError;

const AUTH_DATE_TTL_SECS: i64 = 86400;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthData {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub auth_date: i64,
    pub hash: String,
}

pub type TelegramUserInfo = AuthData;

fn check_string(data: &AuthData) -> String {
    let mut pairs: Vec<(String, String)> = Vec::new();

    pairs.push(("id".to_string(), data.id.to_string()));
    if let Some(v) = &data.first_name {
        pairs.push(("first_name".to_string(), v.clone()));
    }
    if let Some(v) = &data.last_name {
        pairs.push(("last_name".to_string(), v.clone()));
    }
    if let Some(v) = &data.username {
        pairs.push(("username".to_string(), v.clone()));
    }
    if let Some(v) = &data.photo_url {
        pairs.push(("photo_url".to_string(), v.clone()));
    }
    pairs.push(("auth_date".to_string(), data.auth_date.to_string()));

    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn validate_auth_data(data: &AuthData, bot_token: &[u8]) -> Result<(), OAuthError> {
    if bot_token.is_empty() {
        return Err(OAuthError::Telegram(
            "telegram bot token is not provided".into(),
        ));
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| OAuthError::Telegram(format!("system clock error: {}", e)))?
        .as_secs() as i64;

    if now - data.auth_date > AUTH_DATE_TTL_SECS {
        return Err(OAuthError::Telegram("auth date is expired".into()));
    }

    let check_str = check_string(data);

    let key = sha2::Sha256::digest(bot_token);
    let mut mac = Hmac::<Sha256>::new_from_slice(&key)
        .map_err(|e| OAuthError::Telegram(format!("HMAC key error: {}", e)))?;
    mac.update(check_str.as_bytes());
    let computed = hex::encode(mac.finalize().into_bytes());

    if data.hash != computed {
        return Err(OAuthError::Telegram("hash is not valid".into()));
    }

    Ok(())
}

pub fn parse_and_validate_auth_data(
    json_bytes: &[u8],
    bot_token: &[u8],
) -> Result<AuthData, OAuthError> {
    let data: AuthData = serde_json::from_slice(json_bytes)
        .map_err(|e| OAuthError::Telegram(format!("json parse error: {}", e)))?;
    validate_auth_data(&data, bot_token)?;
    Ok(data)
}

pub fn parse_base64_and_validate(
    base64_str: &str,
    bot_token: &[u8],
) -> Result<AuthData, OAuthError> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(base64_str.as_bytes())
        .or_else(|_| {
            base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(base64_str.as_bytes())
        })
        .map_err(|e| OAuthError::Telegram(format!("base64 decode error: {}", e)))?;

    parse_and_validate_auth_data(&decoded, bot_token)
}
