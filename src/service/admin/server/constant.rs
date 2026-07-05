//! Supported protocol name constants.

pub const PROTOCOL_SHADOWTLS: &str = "shadowtls";
pub const PROTOCOL_VLESS: &str = "vless";
pub const PROTOCOL_TROJAN: &str = "trojan";
pub const PROTOCOL_HYSTERIA2: &str = "hysteria2";
pub const PROTOCOL_TUIC: &str = "tuic";
pub const PROTOCOL_VMESS: &str = "vmess";
pub const PROTOCOL_SS: &str = "ss";

/// Full list of supported protocol names exposed via `get_server_protocols`.
pub const SUPPORTED_PROTOCOLS: &[&str] = &[
    PROTOCOL_SHADOWTLS,
    PROTOCOL_VLESS,
    PROTOCOL_TROJAN,
    PROTOCOL_HYSTERIA2,
    PROTOCOL_TUIC,
    PROTOCOL_VMESS,
    PROTOCOL_SS,
];
