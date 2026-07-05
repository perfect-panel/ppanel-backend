use std::fmt;

#[derive(Debug)]
pub enum OAuthError {
    Config(String),
    Telegram(String),
    Arctic(arctic_oauth::Error),
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuthError::Config(msg) => write!(f, "OAuth config error: {}", msg),
            OAuthError::Telegram(msg) => write!(f, "Telegram OAuth error: {}", msg),
            OAuthError::Arctic(e) => write!(f, "OAuth error: {}", e),
        }
    }
}

impl std::error::Error for OAuthError {}

impl From<arctic_oauth::Error> for OAuthError {
    fn from(e: arctic_oauth::Error) -> Self {
        OAuthError::Arctic(e)
    }
}
