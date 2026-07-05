use crate::model::entity::ticket::{Follow, Ticket};
use crate::repository::audit;
use crate::repository::ticket::{TicketDetails, TicketRepo};

pub struct PgTicketRepo {
    pool: sqlx::PgPool,
}

impl PgTicketRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TicketRepo for PgTicketRepo {
    async fn insert(&self, data: &Ticket) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "INSERT INTO ticket (title, description, user_id, status, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.user_id)
        .bind(data.status)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>("SELECT * FROM ticket WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Ticket) -> Result<Ticket, sqlx::Error> {
        sqlx::query_as::<_, Ticket>(
            "UPDATE ticket SET title = $1, description = $2, user_id = $3, status = $4, updated_at = $5
             WHERE id = $6 RETURNING *",
        )
        .bind(&data.title)
        .bind(&data.description)
        .bind(data.user_id)
        .bind(data.status)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM ticket WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_follow(&self, data: &Follow) -> Result<Follow, sqlx::Error> {
        sqlx::query_as::<_, Follow>(
            r#"INSERT INTO ticket_follow (ticket_id, "from", "type", content, created_at)
               VALUES ($1, $2, $3, $4, $5)
               RETURNING *"#,
        )
        .bind(data.ticket_id)
        .bind(&data.from)
        .bind(data.type_)
        .bind(&data.content)
        .bind(data.created_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_follows_by_ticket(&self, ticket_id: i64) -> Result<Vec<Follow>, sqlx::Error> {
        sqlx::query_as::<_, Follow>(
            "SELECT * FROM ticket_follow WHERE ticket_id = $1 ORDER BY created_at ASC",
        )
        .bind(ticket_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_ticket_detail(&self, id: i64) -> Result<TicketDetails, sqlx::Error> {
        sqlx::query_as::<_, TicketDetails>("SELECT * FROM ticket WHERE id = $1")
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
        let mut idx = 0u32;
        if user_id > 0 {
            idx += 1;
            clauses.push(format!("user_id = ${}", idx));
        }
        if status.is_some() {
            idx += 1;
            clauses.push(format!("status = ${}", idx));
        } else {
            idx += 1;
            clauses.push(format!("status != ${}", idx));
        }
        if search.is_some() {
            idx += 1;
            clauses.push(format!("(title ILIKE ${} OR description ILIKE ${})", idx, idx));
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

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
        if let Some(s) = search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM ticket {} ORDER BY id DESC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
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
        if let Some(s) = search {
            list_q = list_q.bind(format!("%{}%", s));
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
            sqlx::query("UPDATE ticket SET status = $1 WHERE id = $2 AND user_id = $3")
                .bind(status)
                .bind(id)
                .bind(user_id)
                .execute(&self.pool)
                .await?
        } else {
            sqlx::query("UPDATE ticket SET status = $1 WHERE id = $2")
                .bind(status)
                .bind(id)
                .execute(&self.pool)
                .await?
        };
        Ok(res.rows_affected())
    }

    async fn query_wait_reply_total(&self) -> Result<i64, sqlx::Error> {
        let (total,) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM ticket WHERE status = 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(total)
    }
}
