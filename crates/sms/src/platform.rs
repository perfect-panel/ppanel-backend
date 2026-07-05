use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    AlibabaCloud,
    Smsbao,
    Abosend,
    Twilio,
}

impl Platform {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "AlibabaCloud" => Some(Platform::AlibabaCloud),
            "smsbao" => Some(Platform::Smsbao),
            "abosend" => Some(Platform::Abosend),
            "twilio" => Some(Platform::Twilio),
            _ => None,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Platform::AlibabaCloud => "AlibabaCloud",
            Platform::Smsbao => "smsbao",
            Platform::Abosend => "abosend",
            Platform::Twilio => "twilio",
        };
        write!(f, "{}", s)
    }
}
