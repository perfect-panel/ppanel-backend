use crate::model::entity::node::{Node, Server, ServerConfigOverride};

#[derive(Debug, Default)]
pub struct ServerFilter {
    pub page: i64,
    pub size: i64,
    pub ids: Vec<i64>,
    pub search: Option<String>,
}

#[derive(Debug, Default)]
pub struct NodeFilter {
    pub page: i64,
    pub size: i64,
    pub node_ids: Vec<i64>,
    pub server_ids: Vec<i64>,
    pub tags: Vec<String>,
    pub search: Option<String>,
    pub protocol: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SortItem {
    pub id: i64,
    pub sort: i64,
}

#[async_trait::async_trait]
pub trait NodeRepo: Send + Sync {
    async fn insert_server(&self, data: &Server) -> Result<Server, sqlx::Error>;
    async fn find_one_server(&self, id: i64) -> Result<Server, sqlx::Error>;
    async fn update_server(&self, data: &Server) -> Result<Server, sqlx::Error>;
    async fn delete_server(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn insert_node(&self, data: &Node) -> Result<Node, sqlx::Error>;
    async fn find_one_node(&self, id: i64) -> Result<Node, sqlx::Error>;
    async fn update_node(&self, data: &Node) -> Result<Node, sqlx::Error>;
    async fn delete_node(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn insert_override(&self, data: &ServerConfigOverride) -> Result<ServerConfigOverride, sqlx::Error>;
    async fn find_one_override(&self, id: i64) -> Result<ServerConfigOverride, sqlx::Error>;
    async fn find_override_by_server(
        &self,
        server_id: i64,
    ) -> Result<Option<ServerConfigOverride>, sqlx::Error>;
    async fn update_override(&self, data: &ServerConfigOverride) -> Result<ServerConfigOverride, sqlx::Error>;
    async fn delete_override(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn filter_server_list(&self, filter: &ServerFilter) -> Result<(i64, Vec<Server>), sqlx::Error>;
    async fn query_server_sorts(&self) -> Result<Vec<SortItem>, sqlx::Error>;
    async fn update_server_sort(&self, id: i64, sort: i64) -> Result<(), sqlx::Error>;
    async fn count_servers_by_report_status(&self, cutoff: i64) -> Result<(i64, i64), sqlx::Error>;
    async fn query_server_addresses(&self) -> Result<Vec<String>, sqlx::Error>;
    async fn filter_node_list(
        &self,
        filter: &NodeFilter,
        preload_server: bool,
    ) -> Result<(i64, Vec<Node>), sqlx::Error>;
    async fn query_node_sorts(&self) -> Result<Vec<SortItem>, sqlx::Error>;
    async fn update_node_sort(&self, id: i64, sort: i64) -> Result<(), sqlx::Error>;
    async fn query_node_tags(&self) -> Result<Vec<String>, sqlx::Error>;
    async fn count_enabled_nodes(&self) -> Result<i64, sqlx::Error>;
    async fn query_enabled_node_protocols(&self) -> Result<Vec<String>, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
