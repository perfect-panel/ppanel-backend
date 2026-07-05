//! API DTO types — node domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::misc::TimePeriod;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNodeRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub port: u16,
    pub address: String,
    pub server_id: i64,
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteNodeRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterNodeListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterNodeListResponse {
    pub total: i64,
    pub list: Vec<Node>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: i64,
    pub name: String,
    pub tags: Vec<String>,
    pub port: u16,
    pub address: String,
    pub server_id: i64,
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<i32>,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub node_secret: String,
    pub node_pull_interval: i64,
    pub node_push_interval: i64,
    pub traffic_report_threshold: i64,
    pub ip_strategy: String,
    pub dns: Vec<NodeDNS>,
    pub block: Vec<String>,
    pub outbound: Vec<NodeOutbound>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDNS {
    pub proto: String,
    pub address: String,
    pub domains: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeOutbound {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    pub password: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cipher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    #[serde(default)]
    pub allow_insecure: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(default)]
    pub uot: bool,
    #[serde(default)]
    pub uot_version: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub congestion_controller: Option<String>,
    #[serde(default)]
    pub udp_stream: bool,
    #[serde(default)]
    pub reduce_rtt: bool,
    #[serde(default)]
    pub heartbeat: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reality_short_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spider_x: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_settings: Option<String>,
    pub rules: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNodeConfigValues {
    pub ip_strategy: String,
    pub dns: Vec<NodeDNS>,
    pub block: Vec<String>,
    pub outbound: Vec<NodeOutbound>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNodeConfigOverride {
    pub inherit_ip_strategy: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_strategy: Option<String>,
    pub inherit_dns: bool,
    pub dns: Vec<NodeDNS>,
    pub inherit_block: bool,
    pub block: Vec<String>,
    pub inherit_outbound: bool,
    pub outbound: Vec<NodeOutbound>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRelay {
    pub host: String,
    pub port: i32,
    pub prefix: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryNodeTagResponse {
    pub tags: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerNodeConfigRequest {
    pub server_id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerNodeConfigResponse {
    pub global: ServerNodeConfigValues,
    pub r#override: ServerNodeConfigOverride,
    pub effective: ServerNodeConfigValues,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServerNodeConfigRequest {
    pub server_id: i64,
    #[serde(flatten)]
    pub override_config: ServerNodeConfigOverride,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToggleNodeStatusRequest {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNodeRequest {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub port: u16,
    pub address: String,
    pub server_id: i64,
    pub protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
