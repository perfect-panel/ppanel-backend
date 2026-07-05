use crate::model::entity::traffic::{
    ServerTrafficRanking, TotalTraffic, TrafficLog, UserTrafficRanking,
};

#[derive(Debug, Default)]
pub struct TrafficLogDetailsFilter {
    pub server_id: i64,
    pub user_id: i64,
    pub subscribe_id: i64,
    pub start: i64,
    pub end: i64,
    pub page: i64,
    pub size: i64,
}

#[async_trait::async_trait]
pub trait TrafficRepo: Send + Sync {
    async fn insert(&self, data: &TrafficLog) -> Result<TrafficLog, sqlx::Error>;
    async fn bulk_insert(&self, rows: &[TrafficLog]) -> Result<u64, sqlx::Error>;
    async fn query_server_traffic_by_day(
        &self,
        server_id: i64,
        date: i64,
    ) -> Result<TotalTraffic, sqlx::Error>;
    async fn query_traffic_by_day(&self, date: i64) -> Result<TotalTraffic, sqlx::Error>;
    async fn query_traffic_by_monthly(&self, date: i64) -> Result<TotalTraffic, sqlx::Error>;
    async fn query_traffic_summary(&self, start: i64, end: i64) -> Result<TotalTraffic, sqlx::Error>;
    async fn top_servers_traffic_by_day(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<ServerTrafficRanking>, sqlx::Error>;
    async fn top_servers_traffic_by_monthly(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<ServerTrafficRanking>, sqlx::Error>;
    async fn top_users_traffic_by_day(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<UserTrafficRanking>, sqlx::Error>;
    async fn top_users_traffic_by_monthly(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<UserTrafficRanking>, sqlx::Error>;
    async fn query_server_traffic_ranking(
        &self,
        start: i64,
        end: i64,
    ) -> Result<Vec<ServerTrafficRanking>, sqlx::Error>;
    async fn query_user_traffic_ranking(
        &self,
        start: i64,
        end: i64,
    ) -> Result<Vec<UserTrafficRanking>, sqlx::Error>;
    async fn query_traffic_log_page_list(
        &self,
        user_id: i64,
        subscribe_id: i64,
        page: i64,
        size: i64,
    ) -> Result<(Vec<TrafficLog>, i64), sqlx::Error>;
    async fn query_traffic_log_details(
        &self,
        filter: &TrafficLogDetailsFilter,
    ) -> Result<(Vec<TrafficLog>, i64), sqlx::Error>;
    async fn delete_before(&self, end: i64) -> Result<u64, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
