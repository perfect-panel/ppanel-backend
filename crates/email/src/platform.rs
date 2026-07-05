use std::collections::HashMap;
use std::str::FromStr;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Smtp,
    Unsupported,
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "smtp" => Ok(Platform::Smtp),
            _ => Ok(Platform::Unsupported),
        }
    }
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Smtp => "smtp",
            Platform::Unsupported => "unsupported",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformInfo {
    pub platform: String,
    pub platform_url: String,
    pub platform_field_description: HashMap<String, String>,
}

pub fn get_supported_platforms() -> Vec<PlatformInfo> {
    let mut desc = HashMap::new();
    desc.insert("host".into(), "host".into());
    desc.insert("port".into(), "port".into());
    desc.insert("user".into(), "user".into());
    desc.insert("pass".into(), "pass".into());
    desc.insert("from".into(), "from".into());
    desc.insert("reply_to".into(), "reply_to".into());
    desc.insert("ssl".into(), "ssl".into());

    vec![PlatformInfo {
        platform: "smtp".into(),
        platform_url: String::new(),
        platform_field_description: desc,
    }]
}
