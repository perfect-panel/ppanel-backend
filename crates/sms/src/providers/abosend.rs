use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;

use crate::config::SmsConfig;
use crate::sender::Sender;

const BASE_URL: &str = "https://smsapi.abosend.com";

pub struct AbosendSender {
    config: SmsConfig,
    client: Client,
    base_url: String,
}

impl AbosendSender {
    pub fn new(config: SmsConfig) -> Self {
        let base_url = if config.api_domain.is_empty() {
            BASE_URL.to_string()
        } else {
            config.api_domain.clone()
        };
        Self {
            config,
            client: Client::new(),
            base_url,
        }
    }
}

#[derive(Debug, Deserialize)]
struct AbosendResponse {
    code: i32,
    message: String,
}

/// Render a Go-style template string: replace `{{.code}}` with `code`.
fn render_template(template: &str, code: &str) -> String {
    template.replace("{{.code}}", code)
}

/// Compute MD5 hex string (lowercase).
fn md5_hex(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

#[async_trait::async_trait]
impl Sender for AbosendSender {
    async fn send(&self, area: &str, phone: &str, code: &str, _expire: u32) -> anyhow::Result<()> {
        let content = render_template(&self.config.template, code);

        // rand is a 6-digit numeric string; we derive it from current time nanos
        let rand_num = format!(
            "{:06}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos()
                % 1_000_000
        );

        // sign = md5(orgCode + content + rand + md5key), uppercase
        let sign_input = format!(
            "{}{}{}{}",
            self.config.access, content, rand_num, self.config.secret
        );
        let sign = md5_hex(&sign_input).to_uppercase();

        let body = serde_json::json!({
            "orgCode":    self.config.access,
            "mobileArea": format!("+{}", area),
            "mobiles":    format!("{}{}", area, phone),
            "content":    content,
            "rand":       rand_num,
            "sign":       sign,
        });

        let url = format!("{}/v2/api/sendSMS", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Abosend: HTTP request failed")?;

        let status = resp.status();
        if !status.is_success() {
            anyhow::bail!("Abosend: HTTP {}", status);
        }

        let result: AbosendResponse = resp
            .json()
            .await
            .context("Abosend: failed to parse response")?;

        if result.code != 200 {
            anyhow::bail!(
                "Abosend: send failed, code={}, message={}",
                result.code,
                result.message
            );
        }

        Ok(())
    }
}
