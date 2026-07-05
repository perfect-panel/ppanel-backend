//! API DTO types — server domain.
//! Auto-generated from the monolithic `dto.rs`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::node::{NodeDNS, NodeOutbound};
use super::protocol::Protocol;
use super::user::{UserTraffic, UserTrafficData};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<i32>,
    pub protocols: Vec<Protocol>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteServerRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterServerListRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterServerListResponse {
    pub total: i64,
    pub list: Vec<Server>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerConfigRequest {
    #[serde(flatten)]
    pub common: ServerCommon,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerConfigResponse {
    pub basic: ServerBasic,
    pub protocol: String,
    pub config: Value,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerProtocolsRequest {
    pub id: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerProtocolsResponse {
    pub protocols: Vec<Protocol>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerUserListRequest {
    #[serde(flatten)]
    pub common: ServerCommon,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetServerUserListResponse {
    pub users: Vec<ServerUser>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasMigrateSeverNodeResponse {
    pub has_migrate: bool,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrateServerNodeResponse {
    pub succee: u64,
    pub fail: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineUser {
    #[serde(rename = "uid")]
    pub sid: i64,
    pub ip: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineUsersRequest {
    #[serde(flatten)]
    pub common: ServerCommon,
    pub users: Vec<OnlineUser>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreViewNodeMultiplierResponse {
    pub current_time: String,
    pub ratio: f32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryServerConfigRequest {
    pub server_id: i64,
    pub secret_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryServerConfigResponse {
    pub traffic_report_threshold: i64,
    pub push_interval: i64,
    pub pull_interval: i64,
    pub ip_strategy: String,
    pub dns: Vec<NodeDNS>,
    pub block: Vec<String>,
    pub outbound: Vec<NodeOutbound>,
    pub protocols: Vec<Protocol>,
    pub total: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetSortRequest {
    pub sort: Vec<SortItem>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub country: String,
    pub city: String,
    pub address: String,
    pub sort: i32,
    pub protocols: Vec<Protocol>,
    pub last_reported_at: i64,
    pub status: ServerStatus,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBasic {
    pub push_interval: i64,
    pub pull_interval: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCommon {
    pub protocol: String,
    pub server_id: i64,
    pub secret_key: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerGroup {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerOnlineIP {
    pub ip: String,
    pub protocol: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerOnlineUser {
    pub ip: Vec<ServerOnlineIP>,
    pub user_id: i64,
    pub subscribe: String,
    pub subscribe_id: i64,
    pub traffic: i64,
    pub expired_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPushStatusRequest {
    #[serde(flatten)]
    pub common: ServerCommon,
    pub cpu: f64,
    pub mem: f64,
    pub disk: f64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerPushUserTrafficRequest {
    #[serde(flatten)]
    pub common: ServerCommon,
    pub traffic: Vec<UserTraffic>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerRuleGroup {
    pub id: i64,
    pub icon: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub tags: Vec<String>,
    pub rules: String,
    pub enable: bool,
    pub default: bool,
    pub created_at: i64,
    pub updated_at: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub cpu: f64,
    pub mem: f64,
    pub disk: f64,
    pub protocol: String,
    pub online: Vec<ServerOnlineUser>,
    pub status: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTotalDataResponse {
    pub online_users: i64,
    pub online_servers: i64,
    pub offline_servers: i64,
    pub today_upload: i64,
    pub today_download: i64,
    pub monthly_upload: i64,
    pub monthly_download: i64,
    pub updated_at: i64,
    pub server_traffic_ranking_today: Vec<ServerTrafficData>,
    pub server_traffic_ranking_yesterday: Vec<ServerTrafficData>,
    pub user_traffic_ranking_today: Vec<UserTrafficData>,
    pub user_traffic_ranking_yesterday: Vec<UserTrafficData>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTrafficData {
    pub server_id: i64,
    pub name: String,
    pub upload: i64,
    pub download: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerUser {
    pub id: i64,
    pub uuid: String,
    pub speed_limit: i64,
    pub device_limit: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortItem {
    pub id: i64,
    pub sort: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServerRequest {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<i32>,
    pub protocols: Vec<Protocol>,
}
