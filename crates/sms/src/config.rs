use serde::{Deserialize, Serialize};

/// Unified config struct covering all 4 providers.
/// Fields not used by a given provider are simply ignored.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SmsConfig {
    // ── AlibabaCloud ────────────────────────────────────────────────
    /// AccessKeyId
    #[serde(default)]
    pub access: String,
    /// AccessKeySecret / MD5Key / AuthToken / Password
    #[serde(default)]
    pub secret: String,
    /// AlibabaCloud: SignName
    #[serde(default)]
    pub sign_name: String,
    /// AlibabaCloud: Endpoint (default: dysmsapi.ap-southeast-1.aliyuncs.com)
    #[serde(default)]
    pub endpoint: String,
    /// AlibabaCloud: TemplateCode
    #[serde(default)]
    pub template_code: String,

    // ── Smsbao / Abosend / Twilio ────────────────────────────────────
    /// Go template string, e.g. "Your code is {{.code}}"
    #[serde(default)]
    pub template: String,

    // ── Abosend ─────────────────────────────────────────────────────
    /// Override API base URL (default: https://smsapi.abosend.com)
    #[serde(default)]
    pub api_domain: String,

    // ── Twilio ───────────────────────────────────────────────────────
    /// Sending phone number (e.g. "+12015551234")
    #[serde(default)]
    pub phone_number: String,
}
