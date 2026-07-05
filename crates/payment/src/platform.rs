use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Stripe,
    AlipayF2F,
    EPay,
    Balance,
    CryptoSaaS,
    Unsupported,
}

const PLATFORM_NAMES: &[(&str, Platform)] = &[
    ("CryptoSaaS", Platform::CryptoSaaS),
    ("Stripe", Platform::Stripe),
    ("AlipayF2F", Platform::AlipayF2F),
    ("EPay", Platform::EPay),
    ("balance", Platform::Balance),
];

impl Platform {
    pub fn from_str(s: &str) -> Self {
        for &(name, platform) in PLATFORM_NAMES {
            if name == s {
                return platform;
            }
        }
        Platform::Unsupported
    }

    pub fn as_str(&self) -> &'static str {
        for &(name, platform) in PLATFORM_NAMES {
            if platform == *self {
                return name;
            }
        }
        "unsupported"
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Platform {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::from_str(s) {
            Platform::Unsupported => Err(()),
            p => Ok(p),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformInfo {
    pub platform: String,
    pub platform_url: String,
    pub platform_field_description: std::collections::HashMap<String, String>,
}

pub fn get_supported_platforms() -> Vec<PlatformInfo> {
    vec![
        PlatformInfo {
            platform: "Stripe".into(),
            platform_url: "https://stripe.com".into(),
            platform_field_description: {
                let mut m = std::collections::HashMap::new();
                m.insert("public_key".into(), "Publishable key".into());
                m.insert("secret_key".into(), "Secret key".into());
                m.insert("webhook_secret".into(), "Webhook secret".into());
                m.insert("payment".into(), "Payment Method, only supported card/alipay/wechat_pay".into());
                m
            },
        },
        PlatformInfo {
            platform: "AlipayF2F".into(),
            platform_url: "https://alipay.com".into(),
            platform_field_description: {
                let mut m = std::collections::HashMap::new();
                m.insert("app_id".into(), "App ID".into());
                m.insert("private_key".into(), "Private Key".into());
                m.insert("public_key".into(), "Public Key".into());
                m.insert("invoice_name".into(), "Invoice Name".into());
                m.insert("sandbox".into(), "Sandbox Mode".into());
                m
            },
        },
        PlatformInfo {
            platform: "EPay".into(),
            platform_url: String::new(),
            platform_field_description: {
                let mut m = std::collections::HashMap::new();
                m.insert("pid".into(), "PID".into());
                m.insert("url".into(), "URL".into());
                m.insert("key".into(), "Key".into());
                m.insert("type".into(), "Type".into());
                m
            },
        },
        PlatformInfo {
            platform: "CryptoSaaS".into(),
            platform_url: "https://t.me/CryptoSaaSBot".into(),
            platform_field_description: {
                let mut m = std::collections::HashMap::new();
                m.insert("endpoint".into(), "API Endpoint".into());
                m.insert("account_id".into(), "Account ID".into());
                m.insert("secret_key".into(), "Secret Key".into());
                m
            },
        },
    ]
}

use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_parse() {
        assert_eq!(Platform::from_str("Stripe"), Platform::Stripe);
        assert_eq!(Platform::from_str("AlipayF2F"), Platform::AlipayF2F);
        assert_eq!(Platform::from_str("EPay"), Platform::EPay);
        assert_eq!(Platform::from_str("balance"), Platform::Balance);
        assert_eq!(Platform::from_str("CryptoSaaS"), Platform::CryptoSaaS);
        assert_eq!(Platform::from_str("unknown"), Platform::Unsupported);
    }

    #[test]
    fn test_platform_display() {
        assert_eq!(Platform::Stripe.to_string(), "Stripe");
        assert_eq!(Platform::Unsupported.to_string(), "unsupported");
    }

    #[test]
    fn test_get_supported_platforms() {
        let platforms = get_supported_platforms();
        assert_eq!(platforms.len(), 4);
        assert!(platforms.iter().any(|p| p.platform == "Stripe"));
        assert!(platforms.iter().any(|p| p.platform == "EPay"));
    }
}
