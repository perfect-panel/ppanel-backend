use serde::{Deserialize, Serialize};

/// Node (`nodes` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub tags: String,
    pub port: i32,
    pub address: String,
    pub server_id: i64,
    pub protocol: String,
    pub enabled: Option<bool>,
    pub sort: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Server (`servers` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub sort: i32,
    pub protocols: String,
    pub last_reported_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Protocol configuration (serialised within `Server.protocols`).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Protocol {
    #[serde(rename = "type")]
    pub type_: String,
    pub port: i32,
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

/// Server config override (`server_config_overrides` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerConfigOverride {
    pub id: i64,
    pub server_id: i64,
    pub ip_strategy: Option<String>,
    pub dns: Option<String>,
    pub block: Option<String>,
    pub outbound: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}
