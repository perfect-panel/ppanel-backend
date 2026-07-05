/// Supported proxy protocols, ported from `server/internal/config/protocol.go`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Shadowsocks,
    Trojan,
    Vmess,
    Vless,
}

impl Protocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Protocol::Shadowsocks => "shadowsocks",
            Protocol::Trojan => "trojan",
            Protocol::Vmess => "vmess",
            Protocol::Vless => "vless",
        }
    }
}

impl TryFrom<&str> for Protocol {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "shadowsocks" => Ok(Protocol::Shadowsocks),
            "trojan" => Ok(Protocol::Trojan),
            "vmess" => Ok(Protocol::Vmess),
            "vless" => Ok(Protocol::Vless),
            other => Err(format!("unknown protocol: {other}")),
        }
    }
}
