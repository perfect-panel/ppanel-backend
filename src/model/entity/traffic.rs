use serde::{Deserialize, Serialize};

/// Traffic log entry (`traffic_log` table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TrafficLog {
    pub id: i64,
    pub server_id: i64,
    pub user_id: i64,
    pub subscribe_id: i64,
    pub download: i64,
    pub upload: i64,
    pub timestamp: i64,
}

/// Aggregated traffic totals (used in queries, not a table).
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct TotalTraffic {
    pub download: i64,
    pub upload: i64,
}

/// Server traffic ranking row.
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct ServerTrafficRanking {
    pub server_id: i64,
    pub download: i64,
    pub upload: i64,
    pub total: i64,
}

/// User traffic ranking row.
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserTrafficRanking {
    pub user_id: i64,
    pub subscribe_id: i64,
    pub download: i64,
    pub upload: i64,
    pub total: i64,
}
