use crate::model::entity::payment::Payment;
use crate::repository::audit;
use crate::repository::payment::{PaymentFilter, PaymentRepo};

pub struct PgPaymentRepo {
    pool: sqlx::PgPool,
}

impl PgPaymentRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl PaymentRepo for PgPaymentRepo {
    async fn insert(&self, data: &Payment) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>(
            r#"INSERT INTO payment (name, platform, icon, domain, config, description, fee_mode, fee_percent, fee_amount, sort, enable, token, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
               RETURNING *"#,
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
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payment WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_by_token(&self, token: &str) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payment WHERE token = $1")
            .bind(token)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Payment) -> Result<Payment, sqlx::Error> {
        sqlx::query_as::<_, Payment>(
            r#"UPDATE payment SET name = $1, platform = $2, icon = $3, domain = $4, config = $5,
               description = $6, fee_mode = $7, fee_percent = $8, fee_amount = $9, sort = $10,
               enable = $11, token = $12, updated_at = $13
               WHERE id = $14 RETURNING *"#,
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
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM payment WHERE id = $1")
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
        let mut idx = 0u32;
        if let Some(f) = filter {
            if f.enable.is_some() {
                idx += 1;
                clauses.push(format!("enable = ${}", idx));
            }
            if f.platform.is_some() {
                idx += 1;
                clauses.push(format!("platform = ${}", idx));
            }
            if f.search.is_some() {
                idx += 1;
                clauses.push(format!("name ILIKE ${}", idx));
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
            "SELECT * FROM payment {} ORDER BY sort ASC, id ASC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
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
