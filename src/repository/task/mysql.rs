use crate::model::entity::task::{EmailScope, Task};
use crate::repository::audit;
use crate::repository::normalize_page;
use crate::repository::task::{TaskFilter, TaskRepo};

pub struct MySqlTaskRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlTaskRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TaskRepo for MySqlTaskRepo {
    async fn insert(&self, data: &Task) -> Result<Task, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO task (`type`, scope, content, status, errors, total, current, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(data.type_)
        .bind(&data.scope)
        .bind(&data.content)
        .bind(data.status)
        .bind(&data.errors)
        .bind(data.total)
        .bind(data.current)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Task>("SELECT * FROM task WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>("SELECT * FROM task WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_by_type(&self, id: i64, type_: i16) -> Result<Task, sqlx::Error> {
        sqlx::query_as::<_, Task>(
            "SELECT * FROM task WHERE id = ? AND `type` = ?",
        )
        .bind(id)
        .bind(type_)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, data: &Task) -> Result<Task, sqlx::Error> {
        sqlx::query(
            "UPDATE task SET scope = ?, content = ?, status = ?, errors = ?, total = ?, current = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.scope)
        .bind(&data.content)
        .bind(data.status)
        .bind(&data.errors)
        .bind(data.total)
        .bind(data.current)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Task>("SELECT * FROM task WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_status(&self, id: i64, status: i16) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("UPDATE task SET status = ? WHERE id = ?")
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM task WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn query_task_list(
        &self,
        filter: &TaskFilter,
    ) -> Result<(i64, Vec<Task>), sqlx::Error> {
        let mut page = filter.page;
        let mut size = filter.size;
        normalize_page(&mut page, &mut size);

        let mut clauses = Vec::new();
        if filter.type_ != 0 {
            clauses.push("`type` = ?".to_string());
        }
        if filter.status.is_some() {
            clauses.push("status = ?".to_string());
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        if let Some(scope) = filter.scope {
            let sql = format!("SELECT * FROM task {} ORDER BY created_at DESC", where_str);
            let mut q = sqlx::query_as::<_, Task>(audit(&sql));
            if filter.type_ != 0 {
                q = q.bind(filter.type_);
            }
            if let Some(st) = filter.status {
                q = q.bind(st);
            }
            let all = q.fetch_all(&self.pool).await?;
            let mut filtered: Vec<Task> = Vec::with_capacity(all.len());
            for item in all {
                let matches = item
                    .scope
                    .as_ref()
                    .and_then(|s| serde_json::from_str::<EmailScope>(s).ok())
                    .map(|e| e.type_ == scope)
                    .unwrap_or(false);
                if matches {
                    filtered.push(item);
                }
            }
            let total = filtered.len() as i64;
            let start = ((page - 1) * size) as usize;
            let end = (start + size as usize).min(filtered.len());
            let items = if start >= filtered.len() {
                Vec::new()
            } else {
                filtered[start..end].to_vec()
            };
            return Ok((total, items));
        }

        let count_sql = format!("SELECT COUNT(*) FROM task {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if filter.type_ != 0 {
            count_q = count_q.bind(filter.type_);
        }
        if let Some(st) = filter.status {
            count_q = count_q.bind(st);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let offset = (page - 1) * size;
        let list_sql = format!(
            "SELECT * FROM task {} ORDER BY created_at DESC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Task>(audit(&list_sql));
        if filter.type_ != 0 {
            list_q = list_q.bind(filter.type_);
        }
        if let Some(st) = filter.status {
            list_q = list_q.bind(st);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }
}
