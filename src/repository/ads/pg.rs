use crate::model::entity::ads::Ads;
use crate::repository::ads::AdsRepo;
use crate::repository::audit;

pub struct PgAdsRepo {
    pool: sqlx::PgPool,
}

impl PgAdsRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AdsRepo for PgAdsRepo {
    async fn insert(&self, data: &Ads) -> Result<Ads, sqlx::Error> {
        sqlx::query_as::<_, Ads>(
            r#"INSERT INTO ads (title, "type", content, description, target_url, start_time, end_time, status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
               RETURNING *"#,
        )
        .bind(&data.title)
        .bind(&data.type_)
        .bind(&data.content)
        .bind(&data.description)
        .bind(&data.target_url)
        .bind(data.start_time)
        .bind(data.end_time)
        .bind(data.status)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Ads, sqlx::Error> {
        sqlx::query_as::<_, Ads>("SELECT * FROM ads WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Ads) -> Result<Ads, sqlx::Error> {
        sqlx::query_as::<_, Ads>(
            r#"UPDATE ads SET title = $1, content = $2, description = $3, target_url = $4,
               start_time = $5, end_time = $6, status = $7, updated_at = $8
               WHERE id = $9 RETURNING *"#,
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.description)
        .bind(&data.target_url)
        .bind(data.start_time)
        .bind(data.end_time)
        .bind(data.status)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM ads WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn get_list_by_page(
        &self,
        page: i64,
        size: i64,
        status: Option<i32>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Ads>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        let mut idx = 0u32;
        if status.is_some() {
            idx += 1;
            clauses.push(format!("status = ${}", idx));
        }
        if search.is_some() {
            idx += 1;
            clauses.push(format!("(title ILIKE ${} OR content ILIKE ${})", idx, idx));
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM ads {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(v) = status {
            count_q = count_q.bind(v);
        }
        if let Some(s) = search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM ads {} ORDER BY id ASC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, Ads>(audit(&list_sql));
        if let Some(v) = status {
            list_q = list_q.bind(v);
        }
        if let Some(s) = search {
            list_q = list_q.bind(format!("%{}%", s));
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }
}
