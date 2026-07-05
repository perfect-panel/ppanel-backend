use crate::model::entity::document::Document;
use crate::repository::document::DocumentRepo;
use crate::repository::audit;

pub struct PgDocumentRepo {
    pool: sqlx::PgPool,
}

impl PgDocumentRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl DocumentRepo for PgDocumentRepo {
    async fn insert(&self, data: &Document) -> Result<Document, sqlx::Error> {
        sqlx::query_as::<_, Document>(
            r#"INSERT INTO document (title, content, tags, "show", created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6)
               RETURNING *"#,
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.tags)
        .bind(data.show)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Document, sqlx::Error> {
        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Document) -> Result<Document, sqlx::Error> {
        sqlx::query_as::<_, Document>(
            r#"UPDATE document SET title = $1, content = $2, tags = $3, "show" = $4, updated_at = $5
               WHERE id = $6 RETURNING *"#,
        )
        .bind(&data.title)
        .bind(&data.content)
        .bind(&data.tags)
        .bind(data.show)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM document WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn query_detail(&self, id: i64) -> Result<Option<Document>, sqlx::Error> {
        sqlx::query_as::<_, Document>("SELECT * FROM document WHERE id = $1")
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
        let mut idx = 0u32;
        if tag.is_some() {
            idx += 1;
            clauses.push(format!("tags ILIKE ${}", idx));
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

        let count_sql = format!("SELECT COUNT(*) FROM document {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(t) = tag {
            count_q = count_q.bind(format!("%{}%", t));
        }
        if let Some(s) = search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM document {} ORDER BY id DESC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, Document>(audit(&list_sql));
        if let Some(t) = tag {
            list_q = list_q.bind(format!("%{}%", t));
        }
        if let Some(s) = search {
            list_q = list_q.bind(format!("%{}%", s));
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn get_all_visible(&self) -> Result<Vec<Document>, sqlx::Error> {
        sqlx::query_as::<_, Document>(r#"SELECT * FROM document WHERE "show" = true ORDER BY id ASC"#)
            .fetch_all(&self.pool)
            .await
    }
}
