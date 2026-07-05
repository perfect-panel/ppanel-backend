//! OAuth 2.0 authentication library wrapping `arctic-oauth` with
//! config types matching the database schema, plus Telegram Login support.
//!
//! # Providers
//!
//! | Provider  | Standard OAuth 2.0 | PKCE     | Tokens        |
//! |-----------|-------------------|----------|---------------|
//! | Google    | ✅ (arctic-oauth) | Required | access + refresh + id_token |
//! | Apple     | ✅ (arctic-oauth) | None     | access + id_token |
//! | Telegram  | ⚠️ Custom HMAC   | N/A      | N/A (stateless) |

pub mod config;
pub mod error;
pub mod telegram;

// Re-export arctic-oauth core types + providers
pub use arctic_oauth::{
    create_code_challenge, decode_id_token, generate_code_verifier, generate_state,
    Apple, AppleOptions, Google, GoogleOptions, OAuth2Tokens,
};
pub use arctic_oauth::{CodeChallengeMethod, Error as ArcticError};
pub use config::{AppleConfig, GoogleConfig, TelegramConfig};
pub use error::OAuthError;
pub use telegram::{parse_and_validate_auth_data, parse_base64_and_validate, validate_auth_data, AuthData};

/// Unified user info extracted from any OAuth provider.
#[derive(Debug, Clone)]
pub struct OAuthUserInfo {
    pub open_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
}

impl OAuthUserInfo {
    pub fn from_google(tokens: &OAuth2Tokens) -> Result<Self, OAuthError> {
        let id_token = tokens.id_token()?;
        let claims = decode_id_token(id_token)?;
        let open_id = claims
            .get("sub")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let email = claims.get("email").and_then(|v| v.as_str()).map(String::from);
        let name = claims.get("name").and_then(|v| v.as_str()).map(String::from);
        let picture = claims
            .get("picture")
            .and_then(|v| v.as_str())
            .map(String::from);
        Ok(Self {
            open_id,
            email,
            name,
            picture,
        })
    }

    pub fn from_apple(tokens: &OAuth2Tokens) -> Result<Self, OAuthError> {
        let id_token = tokens.id_token()?;
        let claims = decode_id_token(id_token)?;
        let open_id = claims
            .get("sub")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let email = claims.get("email").and_then(|v| v.as_str()).map(String::from);
        let name = None;
        let picture = None;
        Ok(Self {
            open_id,
            email,
            name,
            picture,
        })
    }

    pub fn from_telegram(data: &AuthData) -> Self {
        let open_id = data.id.to_string();
        let name = Some(
            [data.first_name.as_deref(), data.last_name.as_deref()]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join(" "),
        )
        .filter(|s| !s.is_empty());
        Self {
            open_id,
            email: None,
            name,
            picture: data.photo_url.clone(),
        }
    }

    pub fn open_id(&self) -> &str {
        &self.open_id
    }
}
