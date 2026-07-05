use crate::model::entity::announcement::Announcement;
use crate::repository::announcement::AnnouncementRepo;
use crate::repository::audit;

pub struct PgAnnouncementRepo {
    pool: sqlx::PgPool,
}

impl PgAnnouncementRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AnnouncementRepo for PgAnnouncementRepo {
    async fn insert(&self, data: &Announcement) -> Result<Announcement, sqlx::Error> {
        sqlx::query_as::<_, Announcement>(
            r#"INSERT INTO announcement (title, content, "show", pinned, popup, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING *"#,
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(data.show)
        .bind(data.pinned)
        .bind(data.popup)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Announcement, sqlx::Error> {
        sqlx::query_as::<_, Announcement>("SELECT * FROM announcement WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Announcement) -> Result<Announcement, sqlx::Error> {
        sqlx::query_as::<_, Announcement>(
            r#"UPDATE announcement SET title = $1, content = $2, "show" = $3, pinned = $4, popup = $5, updated_at = $6
               WHERE id = $7 RETURNING *"#,
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(data.show)
        .bind(data.pinned)
        .bind(data.popup)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM announcement WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn get_list_by_page(
        &self,
        page: i64,
        size: i64,
        show: Option<bool>,
        pinned: Option<bool>,
        popup: Option<bool>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Announcement>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        let mut idx = 0u32;
        if show.is_some() {
            idx += 1;
            clauses.push(format!("\"show\" = ${}", idx));
        }
        if pinned.is_some() {
            idx += 1;
            clauses.push(format!("pinned = ${}", idx));
        }
        if popup.is_some() {
            idx += 1;
            clauses.push(format!("popup = ${}", idx));
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

        let order_by = if pinned.unwrap_or(true) {
            "pinned DESC, id DESC"
        } else {
            "id DESC"
        };

        let count_sql = format!("SELECT COUNT(*) FROM announcement {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(v) = show {
            count_q = count_q.bind(v);
        }
        if let Some(v) = pinned {
            count_q = count_q.bind(v);
        }
        if let Some(v) = popup {
            count_q = count_q.bind(v);
        }
        if let Some(s) = search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM announcement {} ORDER BY {} LIMIT ${} OFFSET ${}",
            where_str,
            order_by,
            idx + 1,
            idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, Announcement>(audit(&list_sql));
        if let Some(v) = show {
            list_q = list_q.bind(v);
        }
        if let Some(v) = pinned {
            list_q = list_q.bind(v);
        }
        if let Some(v) = popup {
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
