use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::model::entity::log::{RESET_SUBSCRIBE_TYPE_AUTO, ServerTraffic, UserTraffic};
use crate::model::entity::traffic::TrafficLog;
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTrafficEntry {
    #[serde(rename = "uid")]
    pub sid: i64,
    pub upload: i64,
    pub download: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficStatisticsPayload {
    pub server_id: i64,
    pub protocol: String,
    pub logs: Vec<UserTrafficEntry>,
}

pub struct ResetTrafficLogic {
    repos: Arc<Repositories>,
}

impl ResetTrafficLogic {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let now_ms = Utc::now().timestamp_millis();
        let now_ts = Utc::now().timestamp();
        self.reset_by_cycle(3, now_ms, now_ts, "yearly").await;
        self.reset_by_cycle(1, now_ms, now_ts, "first-of-month").await;
        self.reset_by_cycle(2, now_ms, now_ts, "monthly").await;
        Ok(())
    }

    async fn reset_by_cycle(&self, reset_cycle: i64, now_ms: i64, now_ts: i64, label: &str) {
        let sub_ids = match self.repos.subscribe.query_reset_cycle_subscribe_ids(reset_cycle).await {
            Ok(ids) => ids,
            Err(e) => {
                tracing::error!("[ResetTraffic] query_reset_cycle_subscribe_ids({label}) failed: {e}");
                return;
            }
        };
        if sub_ids.is_empty() {
            return;
        }
        let user_sub_ids: Vec<i64> = match reset_cycle {
            1 => match self.repos.user.query_first_reset_subscribe_ids(&sub_ids, now_ts).await {
                Ok(v) => v,
                Err(e) => { tracing::error!("[ResetTraffic] query_first_reset_subscribe_ids failed: {e}"); return; }
            },
            2 => match self.repos.user.query_monthly_reset_subscribe_ids(&sub_ids, now_ms).await {
                Ok(v) => v,
                Err(e) => { tracing::error!("[ResetTraffic] query_monthly_reset_subscribe_ids failed: {e}"); return; }
            },
            3 => match self.repos.user.query_yearly_reset_subscribe_ids(&sub_ids, now_ts).await {
                Ok(v) => v,
                Err(e) => { tracing::error!("[ResetTraffic] query_yearly_reset_subscribe_ids failed: {e}"); return; }
            },
            _ => return,
        };
        if user_sub_ids.is_empty() {
            return;
        }
        if let Err(e) = self.repos.user.reset_subscribe_traffic_by_ids(&user_sub_ids).await {
            tracing::error!("[ResetTraffic] reset_subscribe_traffic_by_ids({label}) failed: {e}");
            return;
        }
        tracing::info!("[ResetTraffic] {label} reset: {} user-subscribes", user_sub_ids.len());
        let subs = match self.repos.user.find_subscribes_by_ids(&user_sub_ids).await {
            Ok(v) => v,
            Err(e) => { tracing::error!("[ResetTraffic] find_subscribes_by_ids({label}) failed: {e}"); return; }
        };
        for sub in &subs {
            Telemetry::reset_subscribe(&self.repos, sub.user_id, RESET_SUBSCRIBE_TYPE_AUTO, None).await;
        }
    }
}

pub struct ServerDataLogic {
    repos: Arc<Repositories>,
    cache: Arc<crate::cache::Cache>,
}

impl ServerDataLogic {
    pub fn new(repos: Arc<Repositories>, cache: Arc<crate::cache::Cache>) -> Self {
        Self { repos, cache }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let now = Utc::now();
        let today_ms = now.timestamp_millis();
        let yesterday_ms = (now - chrono::Duration::days(1)).timestamp_millis();

        let top_servers_today = self.repos.traffic.top_servers_traffic_by_day(today_ms, 10).await.unwrap_or_else(|e| { tracing::error!("[ServerData] top_servers today: {e}"); vec![] });
        let top_users_today = self.repos.traffic.top_users_traffic_by_day(today_ms, 10).await.unwrap_or_else(|e| { tracing::error!("[ServerData] top_users today: {e}"); vec![] });
        let top_servers_yesterday = self.repos.traffic.top_servers_traffic_by_day(yesterday_ms, 10).await.unwrap_or_else(|e| { tracing::error!("[ServerData] top_servers yesterday: {e}"); vec![] });

        let mut server_rank_today: HashMap<u8, ServerTraffic> = HashMap::new();
        for (i, s) in top_servers_today.iter().enumerate().take(10) {
            server_rank_today.insert((i + 1) as u8, ServerTraffic { server_id: s.server_id, upload: s.upload, download: s.download, total: s.total });
        }
        let mut server_rank_yesterday: HashMap<u8, ServerTraffic> = HashMap::new();
        for (i, s) in top_servers_yesterday.iter().enumerate().take(10) {
            server_rank_yesterday.insert((i + 1) as u8, ServerTraffic { server_id: s.server_id, upload: s.upload, download: s.download, total: s.total });
        }
        let mut user_rank_today: HashMap<u8, UserTraffic> = HashMap::new();
        for (i, u) in top_users_today.iter().enumerate().take(10) {
            user_rank_today.insert((i + 1) as u8, UserTraffic { subscribe_id: u.subscribe_id, user_id: u.user_id, upload: u.upload, download: u.download, total: u.total });
        }

        let daily = self.repos.traffic.query_traffic_by_day(today_ms).await.unwrap_or_default();
        let monthly = self.repos.traffic.query_traffic_by_monthly(today_ms).await.unwrap_or_default();

        let snapshot = serde_json::json!({
            "server_traffic_ranking_today": server_rank_today,
            "server_traffic_ranking_yesterday": server_rank_yesterday,
            "user_traffic_ranking_today": user_rank_today,
            "today_upload": daily.upload,
            "today_download": daily.download,
            "monthly_upload": monthly.upload,
            "monthly_download": monthly.download,
            "updated_at": today_ms,
        });
        let json = serde_json::to_string(&snapshot)?;
        if let Err(e) = self.cache.set_ex("server_count", &json, -1).await {
            tracing::error!("[ServerData] cache set failed: {e}");
        }
        Telemetry::server_traffic_rank(&self.repos, server_rank_today).await;
        tracing::info!("[ServerData] snapshot updated");
        Ok(())
    }
}

pub struct StatLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl StatLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let now = Utc::now();
        let yesterday = now - chrono::Duration::days(1);
        let start_ms = yesterday.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis();
        let end_ms = yesterday.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp_millis() + 999;

        let user_traffic = self.repos.traffic.query_user_traffic_ranking(start_ms, end_ms).await
            .map_err(|e| { tracing::error!("[StatLogic] query_user_traffic_ranking: {e}"); e })?;

        let mut user_rank: HashMap<u8, UserTraffic> = HashMap::new();
        for (i, row) in user_traffic.iter().enumerate() {
            let item = UserTraffic { subscribe_id: row.subscribe_id, user_id: row.user_id, upload: row.upload, download: row.download, total: row.total };
            if i < 10 { user_rank.insert((i + 1) as u8, item.clone()); }
            Telemetry::subscribe_traffic(&self.repos, row.subscribe_id, row.download, row.upload).await;
        }
        Telemetry::user_traffic_rank(&self.repos, user_rank).await;

        let server_traffic = self.repos.traffic.query_server_traffic_ranking(start_ms, end_ms).await
            .map_err(|e| { tracing::error!("[StatLogic] query_server_traffic_ranking: {e}"); e })?;

        let mut server_rank: HashMap<u8, ServerTraffic> = HashMap::new();
        for (i, row) in server_traffic.iter().enumerate() {
            let item = ServerTraffic { server_id: row.server_id, upload: row.upload, download: row.download, total: row.total };
            if i < 10 { server_rank.insert((i + 1) as u8, item.clone()); }
            Telemetry::server_traffic(&self.repos, row.server_id, row.download, row.upload).await;
        }
        Telemetry::server_traffic_rank(&self.repos, server_rank).await;

        let summary = self.repos.traffic.query_traffic_summary(start_ms, end_ms).await
            .map_err(|e| { tracing::error!("[StatLogic] query_traffic_summary: {e}"); e })?;
        Telemetry::traffic_stat(&self.repos, summary.upload, summary.download).await;

        if self.config.log.auto_clear {
            let cutoff = (now - chrono::Duration::days(self.config.log.clear_days as i64)).timestamp_millis();
            if let Err(e) = self.repos.traffic.delete_before(cutoff).await {
                tracing::error!("[StatLogic] delete_before: {e}");
            }
        }
        tracing::info!("[StatLogic] daily stat ↑{} ↓{}", summary.upload, summary.download);
        Ok(())
    }
}

pub struct TrafficStatisticsLogic {
    repos: Arc<Repositories>,
    config: Arc<Config>,
}

impl TrafficStatisticsLogic {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    pub async fn execute(&self, payload: TrafficStatisticsPayload) -> anyhow::Result<()> {
        if payload.logs.is_empty() {
            return Ok(());
        }
        let server = match self.repos.node.find_one_server(payload.server_id).await {
            Ok(s) => s,
            Err(e) => { tracing::error!("[TrafficStatistics] find_one_server({}): {e}", payload.server_id); return Ok(()); }
        };
        let ratio = self.resolve_ratio(&server, &payload.protocol);
        let threshold = self.config.node.traffic_report_threshold;
        let now_ms = Utc::now().timestamp_millis();

        for entry in &payload.logs {
            if entry.sid == 0 {
                tracing::warn!("[TrafficStatistics] entry sid=0, skipping");
                continue;
            }
            if entry.upload + entry.download <= threshold {
                continue;
            }
            let sub = match self.repos.user.find_one_subscribe(entry.sid).await {
                Ok(s) => s,
                Err(e) => { tracing::warn!("[TrafficStatistics] find_one_subscribe({}): {e}", entry.sid); continue; }
            };
            let d = (entry.download as f64 * ratio) as i64;
            let u = (entry.upload as f64 * ratio) as i64;
            if let Err(e) = self.repos.user.update_user_subscribe_with_traffic(sub.id, d, u).await {
                tracing::warn!("[TrafficStatistics] update_user_subscribe_with_traffic({}): {e}", sub.id);
                continue;
            }
            let log = TrafficLog {
                id: 0,
                server_id: payload.server_id,
                user_id: sub.user_id,
                subscribe_id: sub.subscribe_id,
                upload: u,
                download: d,
                timestamp: now_ms,
            };
            if let Err(e) = self.repos.traffic.insert(&log).await {
                tracing::warn!("[TrafficStatistics] traffic insert(sid={}): {e}", entry.sid);
            }
            Telemetry::subscribe_traffic(&self.repos, entry.sid, d, u).await;
            Telemetry::server_traffic(&self.repos, payload.server_id, d, u).await;
        }
        Ok(())
    }

    fn resolve_ratio(&self, server: &crate::model::entity::node::Server, protocol: &str) -> f64 {
        let protocols: Vec<crate::model::entity::node::Protocol> =
            serde_json::from_str(&server.protocols).unwrap_or_default();
        for p in &protocols {
            if p.type_.eq_ignore_ascii_case(protocol) && p.ratio > 0.0 {
                return p.ratio;
            }
        }
        1.0
    }
}
