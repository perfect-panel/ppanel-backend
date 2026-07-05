use crate::model::entity::ads::Ads;
use crate::repository::ads::AdsRepo;
use crate::repository::audit;

pub struct MySqlAdsRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlAdsRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AdsRepo for MySqlAdsRepo {
    async fn insert(&self, data: &Ads) -> Result<Ads, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO ads (title, `type`, content, description, target_url, start_time, end_time, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Ads>("SELECT * FROM ads WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Ads, sqlx::Error> {
        sqlx::query_as::<_, Ads>("SELECT * FROM ads WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Ads) -> Result<Ads, sqlx::Error> {
        sqlx::query(
            "UPDATE ads SET title = ?, content = ?, description = ?, target_url = ?,
             start_time = ?, end_time = ?, status = ?, updated_at = ?
             WHERE id = ?",
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
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Ads>("SELECT * FROM ads WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM ads WHERE id = ?")
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
        if status.is_some() {
            clauses.push("status = ?".to_string());
        }
        if search.is_some() {
            clauses.push("(LOWER(title) LIKE LOWER(?) OR LOWER(content) LIKE LOWER(?))".to_string());
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let pattern = search.map(|s| format!("%{}%", s));

        let count_sql = format!("SELECT COUNT(*) FROM ads {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(v) = status {
            count_q = count_q.bind(v);
        }
        if let Some(ref p) = pattern {
            count_q = count_q.bind(p).bind(p);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM ads {} ORDER BY id ASC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Ads>(audit(&list_sql));
        if let Some(v) = status {
            list_q = list_q.bind(v);
        }
        if let Some(ref p) = pattern {
            list_q = list_q.bind(p).bind(p);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }
}
