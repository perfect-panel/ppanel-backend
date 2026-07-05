use crate::model::entity::payment::Payment;
use crate::repository::audit;
use crate::repository::payment::{PaymentFilter, PaymentRepo};

pub struct MySqlPaymentRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlPaymentRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PaymentRepo for MySqlPaymentRepo {
    async fn insert(&self, data: &Payment) -> Result<Payment, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO payment (name, platform, icon, domain, config, description, fee_mode, fee_percent, fee_amount, sort, enable, token, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.name)
        .bind(&data.platform)
        .bind(&data.icon)
        .bind(&data.domain)
        .bind(&data.config)
        .bind(&data.description)
        .bind(data.fee_mode)
        .bind(data.fee_percent)
        .bind(data.fee_amount)
        .bind(data.sort)
        .bind(data.enable)
        .bind(&data.token)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Payment>("SELECT * FROM payment WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payment WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_by_token(&self, token: &str) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payment WHERE token = ?")
            .bind(token)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Payment) -> Result<Payment, sqlx::Error> {
        sqlx::query(
            "UPDATE payment SET name = ?, platform = ?, icon = ?, domain = ?, config = ?,
             description = ?, fee_mode = ?, fee_percent = ?, fee_amount = ?, sort = ?,
             enable = ?, token = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.name)
        .bind(&data.platform)
        .bind(&data.icon)
        .bind(&data.domain)
        .bind(&data.config)
        .bind(&data.description)
        .bind(data.fee_mode)
        .bind(data.fee_percent)
        .bind(data.fee_amount)
        .bind(data.sort)
        .bind(data.enable)
        .bind(&data.token)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Payment>("SELECT * FROM payment WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM payment WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn find_all(&self) -> Result<Vec<Payment>, sqlx::Error> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payment ORDER BY sort ASC, id ASC")
            .fetch_all(&self.pool)
            .await
    }

    async fn find_available_methods(&self) -> Result<Vec<Payment>, sqlx::Error> {
        sqlx::query_as::<_, Payment>(
            "SELECT * FROM payment WHERE enable = true ORDER BY sort ASC, id ASC",
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn find_list_by_page(
        &self,
        page: i64,
        size: i64,
        filter: Option<&PaymentFilter>,
    ) -> Result<(i64, Vec<Payment>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        if let Some(f) = filter {
            if f.enable.is_some() {
                clauses.push("enable = ?".to_string());
            }
            if f.platform.is_some() {
                clauses.push("platform = ?".to_string());
            }
            if f.search.is_some() {
                clauses.push("LOWER(name) LIKE LOWER(?)".to_string());
            }
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM payment {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(f) = filter {
            if let Some(v) = f.enable {
                count_q = count_q.bind(v);
            }
            if let Some(v) = &f.platform {
                count_q = count_q.bind(v);
            }
            if let Some(s) = &f.search {
                count_q = count_q.bind(format!("%{}%", s));
            }
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM payment {} ORDER BY sort ASC, id ASC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Payment>(audit(&list_sql));
        if let Some(f) = filter {
            if let Some(v) = f.enable {
                list_q = list_q.bind(v);
            }
            if let Some(v) = &f.platform {
                list_q = list_q.bind(v);
            }
            if let Some(s) = &f.search {
                list_q = list_q.bind(format!("%{}%", s));
            }
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }
}
