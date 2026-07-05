use crate::model::entity::announcement::Announcement;
use crate::repository::announcement::AnnouncementRepo;
use crate::repository::audit;

pub struct MySqlAnnouncementRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlAnnouncementRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AnnouncementRepo for MySqlAnnouncementRepo {
    async fn insert(&self, data: &Announcement) -> Result<Announcement, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO announcement (title, content, `show`, pinned, popup, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(data.show)
        .bind(data.pinned)
        .bind(data.popup)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Announcement>("SELECT * FROM announcement WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Announcement, sqlx::Error> {
        sqlx::query_as::<_, Announcement>("SELECT * FROM announcement WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Announcement) -> Result<Announcement, sqlx::Error> {
        sqlx::query(
            "UPDATE announcement SET title = ?, content = ?, `show` = ?, pinned = ?, popup = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(data.show)
        .bind(data.pinned)
        .bind(data.popup)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Announcement>("SELECT * FROM announcement WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM announcement WHERE id = ?")
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
        if show.is_some() {
            clauses.push("`show` = ?".to_string());
        }
        if pinned.is_some() {
            clauses.push("pinned = ?".to_string());
        }
        if popup.is_some() {
            clauses.push("popup = ?".to_string());
        }
        if search.is_some() {
            clauses.push("(LOWER(title) LIKE LOWER(?) OR LOWER(content) LIKE LOWER(?))".to_string());
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

        let pattern = search.map(|s| format!("%{}%", s));

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
        if let Some(ref p) = pattern {
            count_q = count_q.bind(p).bind(p);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM announcement {} ORDER BY {} LIMIT ? OFFSET ?",
            where_str,
            order_by,
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
        if let Some(ref p) = pattern {
            list_q = list_q.bind(p).bind(p);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }
}
