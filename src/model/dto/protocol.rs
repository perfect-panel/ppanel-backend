//! API DTO types — protocol domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnyTLS {
    pub port: i32,
    pub security_config: SecurityConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hysteria2 {
    pub port: i32,
    pub hop_ports: String,
    pub hop_interval: i32,
    pub obfs_password: String,
    pub security_config: SecurityConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protocol {
    #[serde(rename = "type")]
    pub type_: String,
    pub port: u16,
    pub enable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    #[serde(default)]
    pub allow_insecure: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_server_addr: Option<String>,
    #[serde(default)]
    pub reality_server_port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_private_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_short_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cipher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(default)]
    pub uot: bool,
    #[serde(default)]
    pub uot_version: i32,
    #[serde(default)]
    pub accept_proxy_protocol: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hop_ports: Option<String>,
    #[serde(default)]
    pub hop_interval: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfs_password: Option<String>,
    #[serde(default)]
    pub disable_sni: bool,
    #[serde(default)]
    pub reduce_rtt: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub udp_relay_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub congestion_controller: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub padding_scheme: Option<String>,
    #[serde(default)]
    pub up_mbps: i32,
    #[serde(default)]
    pub down_mbps: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfs_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfs_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xhttp_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xhttp_extra: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_rtt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_ticket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_server_padding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_private_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_client_padding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_password: Option<String>,
    #[serde(default)]
    pub ech_enable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ech_server_name: Option<String>,
    #[serde(default)]
    pub ratio: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_dns_provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_dns_env: Option<String>,
}


// ─── Security / Send Code / Server ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_server_addr: Option<String>,
    #[serde(default)]
    pub reality_server_port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_private_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_short_id: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shadowsocks {
    pub method: String,
    pub port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_key: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowsocksProtocol {
    pub port: i32,
    pub method: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    pub path: String,
    pub host: String,
    pub service_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trojan {
    pub port: i32,
    pub transport: String,
    pub transport_config: TransportConfig,
    pub security: String,
    pub security_config: SecurityConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrojanProtocol {
    pub host: String,
    pub port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_tls: Option<bool>,
    pub tls_config: String,
    pub network: String,
    pub transport: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tuic {
    pub port: i32,
    pub disable_sni: bool,
    pub reduce_rtt: bool,
    pub udp_relay_mode: String,
    pub congestion_controller: String,
    pub security_config: SecurityConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vless {
    pub port: i32,
    pub flow: String,
    pub transport: String,
    pub transport_config: TransportConfig,
    pub security: String,
    pub security_config: SecurityConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlessProtocol {
    pub host: String,
    pub port: i32,
    pub network: String,
    pub transport: String,
    pub security: String,
    pub security_config: String,
    pub xtls: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vmess {
    pub port: i32,
    pub transport: String,
    pub transport_config: TransportConfig,
    pub security: String,
    pub security_config: SecurityConfig,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmessProtocol {
    pub host: String,
    pub port: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_tls: Option<bool>,
    pub tls_config: String,
    pub network: String,
    pub transport: String,
}
