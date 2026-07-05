use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::config::SmsConfig;
use crate::sender::Sender;

const DEFAULT_ENDPOINT: &str = "dysmsapi.ap-southeast-1.aliyuncs.com";

pub struct AlibabaCloudSender {
    config: SmsConfig,
    client: Client,
}

impl AlibabaCloudSender {
    pub fn new(config: SmsConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

/// Minimal response body from Dysmsapi SendSms
#[derive(Debug, Deserialize)]
struct SendSmsResponse {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
}

/// Build the canonical query string and HMAC-SHA1 signature required by
/// Alibaba Cloud's Dysmsapi (RPC-style, API version 2017-05-25).
///
/// Reference:
/// https://help.aliyun.com/document_detail/101341.html
fn sign_request(
    access_key_id: &str,
    access_key_secret: &str,
    params: &mut Vec<(String, String)>,
) -> anyhow::Result<String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use hmac::{Hmac, Mac};
    use sha1::Sha1;
    #[allow(unused_imports)]
    use sha2::Sha256;

    // Common system parameters
    let nonce = uuid_v4_simple();
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    params.push(("AccessKeyId".to_string(), access_key_id.to_string()));
    params.push(("Format".to_string(), "JSON".to_string()));
    params.push(("SignatureMethod".to_string(), "HMAC-SHA1".to_string()));
    params.push(("SignatureNonce".to_string(), nonce));
    params.push(("SignatureVersion".to_string(), "1.0".to_string()));
    params.push(("Timestamp".to_string(), timestamp));
    params.push(("Version".to_string(), "2017-05-25".to_string()));

    // Sort by key
    params.sort_by(|a, b| a.0.cmp(&b.0));

    // Percent-encode each key=value pair then join with &
    let canonical = params
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                percent_encode(k),
                percent_encode(v)
            )
        })
        .collect::<Vec<_>>()
        .join("&");

    let string_to_sign = format!(
        "GET&{}&{}",
        percent_encode("/"),
        percent_encode(&canonical)
    );

    // Sign with HMAC-SHA1 using "<secret>&" as key
    let signing_key = format!("{}&", access_key_secret);
    let mut mac = Hmac::<Sha1>::new_from_slice(signing_key.as_bytes())
        .context("HMAC-SHA1 init failed")?;
    mac.update(string_to_sign.as_bytes());
    let signature = STANDARD.encode(mac.finalize().into_bytes());

    params.push(("Signature".to_string(), signature.clone()));

    // Rebuild final query
    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    Ok(query)
}

fn percent_encode(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9'
            | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

fn uuid_v4_simple() -> String {
    // Generate a UUID-like random string without external dep
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    format!("{:08x}-{:04x}-4{:03x}", t, t >> 16, t & 0xfff)
}

#[async_trait::async_trait]
impl Sender for AlibabaCloudSender {
    async fn send(&self, area: &str, phone: &str, code: &str, _expire: u32) -> anyhow::Result<()> {
        let endpoint = if self.config.endpoint.is_empty() {
            DEFAULT_ENDPOINT
        } else {
            &self.config.endpoint
        };

        let template_param = json!({ "code": code }).to_string();
        let phone_number = format!("{}{}", area, phone);

        let mut params: Vec<(String, String)> = vec![
            ("Action".to_string(), "SendSms".to_string()),
            ("PhoneNumbers".to_string(), phone_number),
            ("SignName".to_string(), self.config.sign_name.clone()),
            ("TemplateCode".to_string(), self.config.template_code.clone()),
            ("TemplateParam".to_string(), template_param),
        ];

        let query = sign_request(&self.config.access, &self.config.secret, &mut params)?;
        let url = format!("https://{}/{}", endpoint, query);

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .context("AlibabaCloud: HTTP request failed")?;

        let status = resp.status();
        let body = resp.text().await.context("AlibabaCloud: failed to read body")?;

        if !status.is_success() {
            anyhow::bail!("AlibabaCloud: HTTP {} — {}", status, body);
        }

        let result: SendSmsResponse =
            serde_json::from_str(&body).context("AlibabaCloud: failed to parse response")?;

        if result.code != "OK" {
            anyhow::bail!(
                "AlibabaCloud: SendSms failed, code={}, message={}",
                result.code,
                result.message
            );
        }

        Ok(())
    }
}
