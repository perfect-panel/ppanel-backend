use std::str::FromStr;

use crate::platform::Platform;
use crate::smtp::{SmtpClient, SmtpConfig};

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("SMTP transport error: {0}")]
    SmtpTransport(#[from] lettre::transport::smtp::Error),
    #[error("Message build error: {0}")]
    MessageBuild(String),
    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),
    #[error("Config parse error: {0}")]
    ConfigParse(#[from] serde_json::Error),
}

#[async_trait::async_trait]
pub trait Sender: Send + Sync {
    async fn send(&self, to: &[String], subject: &str, body: &str) -> Result<(), EmailError>;
}

pub fn new_sender(
    platform: &str,
    config: &str,
    site_name: &str,
) -> Result<Box<dyn Sender>, EmailError> {
    match Platform::from_str(platform).unwrap_or(Platform::Unsupported) {
        Platform::Smtp => {
            let mut cfg: SmtpConfig = serde_json::from_str(config)?;
            cfg.site_name = site_name.to_string();
            Ok(Box::new(SmtpClient::new(cfg)))
        }
        _ => Err(EmailError::UnsupportedPlatform(platform.to_string())),
    }
}
