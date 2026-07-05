use crate::model::entity::traffic::{
    ServerTrafficRanking, TotalTraffic, TrafficLog, UserTrafficRanking,
};
use crate::repository::audit;
use crate::repository::normalize_page;
use crate::repository::traffic::{TrafficLogDetailsFilter, TrafficRepo};
use chrono::Datelike;

pub struct PgTrafficRepo {
    pool: sqlx::PgPool,
}

impl PgTrafficRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

fn day_range(ts: i64) -> (i64, i64) {
    let secs = ts / 1000;
    let s = secs - secs % 86400;
    (s * 1000, (s + 86400) * 1000)
}

fn month_range(ts: i64) -> (i64, i64) {
    let secs = ts / 1000;
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0).unwrap_or_default();
    let year = dt.year();
    let month = dt.month();
    let start = chrono::NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .unwrap()
        .and_utc();
    let (ny, nm) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let end = chrono::NaiveDate::from_ymd_opt(ny, nm, 1)
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .unwrap()
        .and_utc();
    (start.timestamp_millis(), end.timestamp_millis())
}

const TOTAL_SELECT: &str =
    "COALESCE(SUM(download), 0) AS download, COALESCE(SUM(upload), 0) AS upload";
const SERVER_RANK_SELECT: &str = "server_id, COALESCE(SUM(download + upload), 0) AS total, \
     COALESCE(SUM(download), 0) AS download, COALESCE(SUM(upload), 0) AS upload";
const USER_RANK_SELECT: &str = "user_id, subscribe_id, COALESCE(SUM(download + upload), 0) AS total, \
     COALESCE(SUM(download), 0) AS download, COALESCE(SUM(upload), 0) AS upload";

#[async_trait::async_trait]
impl TrafficRepo for PgTrafficRepo {
    async fn insert(&self, data: &TrafficLog) -> Result<TrafficLog, sqlx::Error> {
        sqlx::query_as::<_, TrafficLog>(
            "INSERT INTO traffic_log (server_id, user_id, subscribe_id, download, upload, timestamp)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(data.server_id)
        .bind(data.user_id)
        .bind(data.subscribe_id)
        .bind(data.download)
        .bind(data.upload)
        .bind(data.timestamp)
        .fetch_one(&self.pool)
        .await
    }

    async fn bulk_insert(&self, rows: &[TrafficLog]) -> Result<u64, sqlx::Error> {
        let mut count = 0u64;
        for row in rows {
            sqlx::query(
                "INSERT INTO traffic_log (server_id, user_id, subscribe_id, download, upload, timestamp)
                 VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(row.server_id)
            .bind(row.user_id)
            .bind(row.subscribe_id)
            .bind(row.download)
            .bind(row.upload)
            .bind(row.timestamp)
            .execute(&self.pool)
            .await?;
            count += 1;
        }
        Ok(count)
    }

    async fn query_server_traffic_by_day(
        &self,
        server_id: i64,
        date: i64,
    ) -> Result<TotalTraffic, sqlx::Error> {
        let (start, end) = day_range(date);
        sqlx::query_as::<_, TotalTraffic>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE server_id = $1 AND timestamp >= $2 AND timestamp < $3",
            TOTAL_SELECT
        )))
        .bind(server_id)
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
    }

    async fn query_traffic_by_day(&self, date: i64) -> Result<TotalTraffic, sqlx::Error> {
        let (start, end) = day_range(date);
        sqlx::query_as::<_, TotalTraffic>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2",
            TOTAL_SELECT
        )))
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
    }

    async fn query_traffic_by_monthly(&self, date: i64) -> Result<TotalTraffic, sqlx::Error> {
        let (start, end) = month_range(date);
        sqlx::query_as::<_, TotalTraffic>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2",
            TOTAL_SELECT
        )))
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
    }

    async fn query_traffic_summary(
        &self,
        start: i64,
        end: i64,
    ) -> Result<TotalTraffic, sqlx::Error> {
        sqlx::query_as::<_, TotalTraffic>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2",
            TOTAL_SELECT
        )))
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
    }

    async fn top_servers_traffic_by_day(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<ServerTrafficRanking>, sqlx::Error> {
        let (start, end) = day_range(date);
        sqlx::query_as::<_, ServerTrafficRanking>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2 \
             GROUP BY server_id ORDER BY total DESC LIMIT $3",
            SERVER_RANK_SELECT
        )))
        .bind(start)
        .bind(end)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    async fn top_servers_traffic_by_monthly(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<ServerTrafficRanking>, sqlx::Error> {
        let (start, end) = month_range(date);
        sqlx::query_as::<_, ServerTrafficRanking>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2 \
             GROUP BY server_id ORDER BY total DESC LIMIT $3",
            SERVER_RANK_SELECT
        )))
        .bind(start)
        .bind(end)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    async fn top_users_traffic_by_day(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<UserTrafficRanking>, sqlx::Error> {
        let (start, end) = day_range(date);
        sqlx::query_as::<_, UserTrafficRanking>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2 \
             GROUP BY user_id, subscribe_id ORDER BY total DESC LIMIT $3",
            USER_RANK_SELECT
        )))
        .bind(start)
        .bind(end)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    async fn top_users_traffic_by_monthly(
        &self,
        date: i64,
        limit: i64,
    ) -> Result<Vec<UserTrafficRanking>, sqlx::Error> {
        let (start, end) = month_range(date);
        sqlx::query_as::<_, UserTrafficRanking>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2 \
             GROUP BY user_id, subscribe_id ORDER BY total DESC LIMIT $3",
            USER_RANK_SELECT
        )))
        .bind(start)
        .bind(end)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_server_traffic_ranking(
        &self,
        start: i64,
        end: i64,
    ) -> Result<Vec<ServerTrafficRanking>, sqlx::Error> {
        sqlx::query_as::<_, ServerTrafficRanking>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2 \
             GROUP BY server_id ORDER BY total DESC",
            SERVER_RANK_SELECT
        )))
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_user_traffic_ranking(
        &self,
        start: i64,
        end: i64,
    ) -> Result<Vec<UserTrafficRanking>, sqlx::Error> {
        sqlx::query_as::<_, UserTrafficRanking>(audit(&format!(
            "SELECT {} FROM traffic_log WHERE timestamp >= $1 AND timestamp < $2 \
             GROUP BY user_id, subscribe_id ORDER BY total DESC",
            USER_RANK_SELECT
        )))
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_traffic_log_page_list(
        &self,
        user_id: i64,
        subscribe_id: i64,
        page: i64,
        size: i64,
    ) -> Result<(Vec<TrafficLog>, i64), sqlx::Error> {
        let offset = (page - 1) * size;
        let (total,) = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM traffic_log WHERE user_id = $1 AND subscribe_id = $2",
        )
        .bind(user_id)
        .bind(subscribe_id)
        .fetch_one(&self.pool)
        .await?;
        let items = sqlx::query_as::<_, TrafficLog>(
            "SELECT * FROM traffic_log WHERE user_id = $1 AND subscribe_id = $2 ORDER BY timestamp DESC LIMIT $3 OFFSET $4",
        )
        .bind(user_id)
        .bind(subscribe_id)
        .bind(size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok((items, total))
    }

    async fn query_traffic_log_details(
        &self,
        filter: &TrafficLogDetailsFilter,
    ) -> Result<(Vec<TrafficLog>, i64), sqlx::Error> {
        let mut page = filter.page;
        let mut size = filter.size;
        normalize_page(&mut page, &mut size);
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        let mut idx = 0u32;
        if filter.server_id != 0 {
            idx += 1;
            clauses.push(format!("server_id = ${}", idx));
        }
        if filter.user_id != 0 {
            idx += 1;
            clauses.push(format!("user_id = ${}", idx));
        }
        if filter.subscribe_id != 0 {
            idx += 1;
            clauses.push(format!("subscribe_id = ${}", idx));
        }
        if filter.start != 0 && filter.end != 0 {
            idx += 1;
            clauses.push(format!("timestamp >= ${}", idx));
            idx += 1;
            clauses.push(format!("timestamp < ${}", idx));
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM traffic_log {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if filter.server_id != 0 {
            count_q = count_q.bind(filter.server_id);
        }
        if filter.user_id != 0 {
            count_q = count_q.bind(filter.user_id);
        }
        if filter.subscribe_id != 0 {
            count_q = count_q.bind(filter.subscribe_id);
        }
        if filter.start != 0 && filter.end != 0 {
            count_q = count_q.bind(filter.start).bind(filter.end);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM traffic_log {} ORDER BY timestamp DESC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, TrafficLog>(audit(&list_sql));
        if filter.server_id != 0 {
            list_q = list_q.bind(filter.server_id);
        }
        if filter.user_id != 0 {
            list_q = list_q.bind(filter.user_id);
        }
        if filter.subscribe_id != 0 {
            list_q = list_q.bind(filter.subscribe_id);
        }
        if filter.start != 0 && filter.end != 0 {
            list_q = list_q.bind(filter.start).bind(filter.end);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((items, total))
    }

    async fn delete_before(&self, end: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM traffic_log WHERE timestamp <= $1")
            .bind(end)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }
}
