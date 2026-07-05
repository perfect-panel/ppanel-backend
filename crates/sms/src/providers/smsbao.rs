use anyhow::Context;
use reqwest::Client;

use crate::config::SmsConfig;
use crate::sender::Sender;

const BASE_URL: &str = "https://api.smsbao.com";

pub struct SmsbaoSender {
    config: SmsConfig,
    client: Client,
}

impl SmsbaoSender {
    pub fn new(config: SmsConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

/// Render a Go-style template: replace `{{.code}}` with `code`.
fn render_template(template: &str, code: &str) -> String {
    template.replace("{{.code}}", code)
}

/// MD5 hex (lowercase).
fn md5_hex(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

/// Map SMSBao numeric response body to an error description.
fn parse_smsbao_error(body: &str) -> anyhow::Result<()> {
    match body.trim() {
        "0" => Ok(()),
        "30" => anyhow::bail!("SMSBao: password error"),
        "40" => anyhow::bail!("SMSBao: account not found"),
        "41" => anyhow::bail!("SMSBao: insufficient balance"),
        "43" => anyhow::bail!("SMSBao: IP address restrictions"),
        "50" => anyhow::bail!("SMSBao: content contains sensitive words"),
        "51" => anyhow::bail!("SMSBao: mobile number is incorrect"),
        other => anyhow::bail!("SMSBao: unknown error code: {}", other),
    }
}

#[async_trait::async_trait]
impl Sender for SmsbaoSender {
    async fn send(&self, area: &str, phone: &str, code: &str, _expire: u32) -> anyhow::Result<()> {
        let content = render_template(&self.config.template, code);
        let password_md5 = md5_hex(&self.config.secret);

        // Domestic (China, area == "86") → /sms, just mobile number
        // International → /wsms, prepend +area to number
        let (api_path, mobile) = if area == "86" {
            ("/sms", phone.to_string())
        } else {
            ("/wsms", format!("+{}{}", area, phone))
        };

        let url = format!("{}{}", BASE_URL, api_path);
        let resp = self
            .client
            .get(&url)
            .query(&[
                ("u", self.config.access.as_str()),
                ("p", &password_md5),
                ("m", &mobile),
                ("c", &content),
            ])
            .send()
            .await
            .context("SMSBao: HTTP request failed")?;

        let status = resp.status();
        let body = resp.text().await.context("SMSBao: failed to read body")?;

        if !status.is_success() {
            anyhow::bail!("SMSBao: HTTP {} — {}", status, body);
        }

        parse_smsbao_error(&body)?;
        Ok(())
    }
}
