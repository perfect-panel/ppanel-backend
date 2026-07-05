use crate::model::entity::document::Document;
use crate::repository::document::DocumentRepo;
use crate::repository::audit;

pub struct MySqlDocumentRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlDocumentRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl DocumentRepo for MySqlDocumentRepo {
    async fn insert(&self, data: &Document) -> Result<Document, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO document (title, content, tags, `show`, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.tags)
        .bind(data.show)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Document, sqlx::Error> {
        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Document) -> Result<Document, sqlx::Error> {
        sqlx::query(
            "UPDATE document SET title = ?, content = ?, tags = ?, `show` = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.tags)
        .bind(data.show)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM document WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn query_detail(&self, id: i64) -> Result<Option<Document>, sqlx::Error> {
        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn query_list(
        &self,
        page: i64,
        size: i64,
        tag: Option<&str>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Document>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        if tag.is_some() {
            clauses.push("LOWER(tags) LIKE LOWER(?)".to_string());
        }
        if search.is_some() {
            clauses.push("(LOWER(title) LIKE LOWER(?) OR LOWER(content) LIKE LOWER(?))".to_string());
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let tag_pattern = tag.map(|t| format!("%{}%", t));
        let search_pattern = search.map(|s| format!("%{}%", s));

        let count_sql = format!("SELECT COUNT(*) FROM document {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(ref p) = tag_pattern {
            count_q = count_q.bind(p);
        }
        if let Some(ref p) = search_pattern {
            count_q = count_q.bind(p).bind(p);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM document {} ORDER BY id DESC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Document>(audit(&list_sql));
        if let Some(ref p) = tag_pattern {
            list_q = list_q.bind(p);
        }
        if let Some(ref p) = search_pattern {
            list_q = list_q.bind(p).bind(p);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn get_all_visible(&self) -> Result<Vec<Document>, sqlx::Error> {
        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE `show` = true ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
    }
}
