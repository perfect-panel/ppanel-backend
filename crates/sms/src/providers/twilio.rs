use anyhow::Context;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use serde::Deserialize;

use crate::config::SmsConfig;
use crate::sender::Sender;

pub struct TwilioSender {
    config: SmsConfig,
    client: Client,
}

impl TwilioSender {
    pub fn new(config: SmsConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

/// Render Go-style template: replace `{{.code}}` with `code`.
fn render_template(template: &str, code: &str) -> String {
    template.replace("{{.code}}", code)
}

/// Twilio Messages API response (partial — only error fields matter)
#[derive(Debug, Deserialize)]
struct TwilioMessageResponse {
    error_code: Option<i32>,
    error_message: Option<String>,
}

#[async_trait::async_trait]
impl Sender for TwilioSender {
    async fn send(&self, area: &str, phone: &str, code: &str, _expire: u32) -> anyhow::Result<()> {
        let to = format!("+{}{}", area, phone);
        let body_text = render_template(&self.config.template, code);

        // Twilio REST API: POST /2010-04-01/Accounts/{AccountSid}/Messages.json
        // Auth: HTTP Basic  (AccountSid : AuthToken)
        let account_sid = &self.config.access;
        let auth_token = &self.config.secret;
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            account_sid
        );

        let credentials = STANDARD.encode(format!("{}:{}", account_sid, auth_token));

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Basic {}", credentials))
            .form(&[
                ("To", to.as_str()),
                ("From", self.config.phone_number.as_str()),
                ("Body", body_text.as_str()),
            ])
            .send()
            .await
            .context("Twilio: HTTP request failed")?;

        let status = resp.status();
        let raw = resp.text().await.context("Twilio: failed to read body")?;

        if !status.is_success() {
            anyhow::bail!("Twilio: HTTP {} — {}", status, raw);
        }

        let result: TwilioMessageResponse =
            serde_json::from_str(&raw).context("Twilio: failed to parse response")?;

        if let Some(err_code) = result.error_code {
            let msg = result.error_message.unwrap_or_default();
            anyhow::bail!("Twilio: send failed, error_code={}, message={}", err_code, msg);
        }

        Ok(())
    }
}
