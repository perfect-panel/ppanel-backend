use crate::model::entity::ticket::{Follow, Ticket};
use crate::repository::audit;
use crate::repository::ticket::{TicketDetails, TicketRepo};

pub struct MySqlTicketRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlTicketRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TicketRepo for MySqlTicketRepo {
    async fn insert(&self, data: &Ticket) -> Result<Ticket, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO ticket (title, description, user_id, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.user_id)
        .bind(data.status)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Ticket>("SELECT * FROM ticket WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>("SELECT * FROM ticket WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Ticket) -> Result<Ticket, sqlx::Error> {
        sqlx::query(
            "UPDATE ticket SET title = ?, description = ?, user_id = ?, status = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.user_id)
        .bind(data.status)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Ticket>("SELECT * FROM ticket WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM ticket WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_follow(&self, data: &Follow) -> Result<Follow, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO ticket_follow (ticket_id, `from`, `type`, content, created_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(data.ticket_id)
        .bind(&data.from)
        .bind(data.type_)
        .bind(&data.content)
        .bind(data.created_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Follow>("SELECT * FROM ticket_follow WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_follows_by_ticket(&self, ticket_id: i64) -> Result<Vec<Follow>, sqlx::Error> {
        sqlx::query_as::<_, Follow>(
            "SELECT * FROM ticket_follow WHERE ticket_id = ? ORDER BY created_at ASC",
        )
        .bind(ticket_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_ticket_detail(&self, id: i64) -> Result<TicketDetails, sqlx::Error> {
        sqlx::query_as::<_, TicketDetails>("SELECT * FROM ticket WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn query_ticket_list(
        &self,
        page: i64,
        size: i64,
        user_id: i64,
        status: Option<i16>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Ticket>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        if user_id > 0 {
            clauses.push("user_id = ?".to_string());
        }
        if status.is_some() {
            clauses.push("status = ?".to_string());
        } else {
            clauses.push("status != ?".to_string());
        }
        if search.is_some() {
            clauses.push(
                "(LOWER(title) LIKE LOWER(?) OR LOWER(description) LIKE LOWER(?))".to_string(),
            );
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let pattern = search.map(|s| format!("%{}%", s));

        let count_sql = format!("SELECT COUNT(*) FROM ticket {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if user_id > 0 {
            count_q = count_q.bind(user_id);
        }
        if let Some(st) = status {
            count_q = count_q.bind(st);
        } else {
            count_q = count_q.bind(4i16);
        }
        if let Some(ref p) = pattern {
            count_q = count_q.bind(p).bind(p);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM ticket {} ORDER BY id DESC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Ticket>(audit(&list_sql));
        if user_id > 0 {
            list_q = list_q.bind(user_id);
        }
        if let Some(st) = status {
            list_q = list_q.bind(st);
        } else {
            list_q = list_q.bind(4i16);
        }
        if let Some(ref p) = pattern {
            list_q = list_q.bind(p).bind(p);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn update_ticket_status(
        &self,
        id: i64,
        user_id: i64,
        status: i16,
    ) -> Result<u64, sqlx::Error> {
        let res = if user_id > 0 {
            sqlx::query("UPDATE ticket SET status = ? WHERE id = ? AND user_id = ?")
                .bind(status)
                .bind(id)
                .bind(user_id)
                .execute(&self.pool)
                .await?
        } else {
            sqlx::query("UPDATE ticket SET status = ? WHERE id = ?")
                .bind(status)
                .bind(id)
                .execute(&self.pool)
                .await?
        };
        Ok(res.rows_affected())
    }

    async fn query_wait_reply_total(&self) -> Result<i64, sqlx::Error> {
        let (total,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM ticket WHERE status = 1")
                .fetch_one(&self.pool)
                .await?;
        Ok(total)
    }
}
