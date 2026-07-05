use crate::model::entity::log::SystemLog;
use crate::repository::log::LogRepo;
use crate::repository::audit;

pub struct PgLogRepo {
    pool: sqlx::PgPool,
}

impl PgLogRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl LogRepo for PgLogRepo {
    async fn insert(&self, data: &SystemLog) -> Result<SystemLog, sqlx::Error> {
        sqlx::query_as::<_, SystemLog>(
            r#"INSERT INTO system_logs ("type", date, object_id, content, created_at)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING *"#,
        )
        .bind(data.type_)
        .bind(&data.date)
        .bind(data.object_id)
        .bind(&data.content)
        .bind(data.created_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<SystemLog, sqlx::Error> {
        sqlx::query_as::<_, SystemLog>("SELECT * FROM system_logs WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn filter_logs(
        &self,
        page: i64,
        size: i64,
        type_: Option<i16>,
        date: Option<&str>,
        object_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<(Vec<SystemLog>, i64), sqlx::Error> {
        let mut page = page;
        let mut size = size;
        crate::repository::normalize_page(&mut page, &mut size);
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        let mut idx = 0u32;
        if type_.is_some() {
            idx += 1;
            clauses.push(format!(r#""type" = ${}"#, idx));
        }
        if date.is_some() {
            idx += 1;
            clauses.push(format!("date = ${}", idx));
        }
        if object_id.is_some() {
            idx += 1;
            clauses.push(format!("object_id = ${}", idx));
        }
        if search.is_some() {
            idx += 1;
            clauses.push(format!("content ILIKE ${}", idx));
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM system_logs {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(v) = type_ {
            count_q = count_q.bind(v);
        }
        if let Some(d) = date {
            count_q = count_q.bind(d);
        }
        if let Some(v) = object_id {
            count_q = count_q.bind(v);
        }
        if let Some(s) = search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM system_logs {} ORDER BY id DESC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, SystemLog>(audit(&list_sql));
        if let Some(v) = type_ {
            list_q = list_q.bind(v);
        }
        if let Some(d) = date {
            list_q = list_q.bind(d);
        }
        if let Some(v) = object_id {
            list_q = list_q.bind(v);
        }
        if let Some(s) = search {
            list_q = list_q.bind(format!("%{}%", s));
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((items, total))
    }

    async fn find_first_by_date_type(&self, date: &str, type_: i16) -> Result<Option<SystemLog>, sqlx::Error> {
        sqlx::query_as::<_, SystemLog>(
            r#"SELECT * FROM system_logs WHERE date = $1 AND "type" = $2 ORDER BY id ASC LIMIT 1"#,
        )
        .bind(date)
        .bind(type_)
        .fetch_optional(&self.pool)
        .await
    }

    async fn find_by_dates_type(&self, dates: &[String], type_: i16) -> Result<Vec<SystemLog>, sqlx::Error> {
        if dates.is_empty() {
            return Ok(Vec::new());
        }
        let placeholders: Vec<String> = (1..=dates.len()).map(|i| format!("${}", i)).collect();
        let sql = format!(
            r#"SELECT * FROM system_logs WHERE date IN ({}) AND "type" = ${} ORDER BY id DESC"#,
            placeholders.join(", "),
            dates.len() + 1,
        );
        let mut q = sqlx::query_as::<_, SystemLog>(audit(&sql));
        for d in dates {
            q = q.bind(d);
        }
        q = q.bind(type_);
        q.fetch_all(&self.pool).await
    }
}
