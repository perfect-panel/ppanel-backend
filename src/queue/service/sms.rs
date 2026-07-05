use std::sync::Arc;

use serde::Deserialize;

use crate::config::Config;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

/// Port of `server/queue/logic/sms/sendSmsLogic.go`.

#[derive(Debug, Clone, Deserialize)]
pub struct SendSmsPayload {
    /// Country / area dial code, e.g. "86"
    #[serde(rename = "TelephoneArea", default)]
    pub telephone_area: String,
    /// Phone number without country code
    #[serde(rename = "Telephone", default)]
    pub telephone: String,
    /// Verification code string to send
    #[serde(rename = "Content", default)]
    pub content: String,
    /// Expiry minutes (passed to some providers)
    #[serde(rename = "Expire", default)]
    pub expire: u32,
    /// Message type tag (used for audit logging)
    #[serde(rename = "Type", default)]
    pub type_: i16,
}

pub struct SendSmsLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl SendSmsLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self, raw: &[u8]) -> anyhow::Result<()> {
        let payload: SendSmsPayload = match serde_json::from_slice(raw) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("[SendSmsLogic] deserialise payload: {e}");
                return Ok(());
            }
        };

        let platform = match sms::Platform::from_str(&self.config.mobile.platform) {
            Some(p) => p,
            None => {
                tracing::error!(
                    "[SendSmsLogic] unsupported SMS platform: {}",
                    self.config.mobile.platform
                );
                return Ok(());
            }
        };

        let sms_config: sms::SmsConfig =
            match serde_json::from_str(&self.config.mobile.platform_config) {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("[SendSmsLogic] parse platform_config: {e}");
                    return Ok(());
                }
            };

        let sender = sms::create_sender(platform, sms_config);

        let to = format!("+{}{}", payload.telephone_area, payload.telephone);
        let status: i16 = match sender
            .send(&payload.telephone_area, &payload.telephone, &payload.content, payload.expire)
            .await
        {
            Ok(()) => {
                tracing::info!("[SendSmsLogic] sent to {to}");
                1
            }
            Err(e) => {
                tracing::error!("[SendSmsLogic] send to {to} failed: {e}");
                2
            }
        };

        Telemetry::mobile_message(
            &self.repos,
            0,
            &to,
            serde_json::json!({ "content": payload.content }),
            &self.config.mobile.platform,
            "",
            status,
        )
        .await;

        Ok(())
    }
}
