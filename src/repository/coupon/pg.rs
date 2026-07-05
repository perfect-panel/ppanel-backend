use crate::model::entity::coupon::Coupon;
use crate::repository::coupon::CouponRepo;
use crate::repository::audit;

pub struct PgCouponRepo {
    pool: sqlx::PgPool,
}

impl PgCouponRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CouponRepo for PgCouponRepo {
    async fn insert(&self, data: &Coupon) -> Result<Coupon, sqlx::Error> {
        sqlx::query_as::<_, Coupon>(
            r#"INSERT INTO coupon (name, code, count, "type", discount, start_time, expire_time, user_limit, subscribe, used_count, enable, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
               RETURNING *"#,
        )
        .bind(&data.name)
        .bind(&data.code)
        .bind(data.count)
        .bind(data.type_)
        .bind(data.discount)
        .bind(data.start_time)
        .bind(data.expire_time)
        .bind(data.user_limit)
        .bind(&data.subscribe)
        .bind(data.used_count)
        .bind(data.enable)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Coupon, sqlx::Error> {
        sqlx::query_as::<_, Coupon>("SELECT * FROM coupon WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_by_code(&self, code: &str) -> Result<Coupon, sqlx::Error> {
        sqlx::query_as::<_, Coupon>("SELECT * FROM coupon WHERE code = $1")
            .bind(code)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Coupon) -> Result<Coupon, sqlx::Error> {
        sqlx::query_as::<_, Coupon>(
            r#"UPDATE coupon SET name = $1, code = $2, count = $3, "type" = $4, discount = $5,
               start_time = $6, expire_time = $7, user_limit = $8, subscribe = $9, used_count = $10,
               enable = $11, updated_at = $12
               WHERE id = $13 RETURNING *"#,
        )
        .bind(&data.name)
        .bind(&data.code)
        .bind(data.count)
        .bind(data.type_)
        .bind(data.discount)
        .bind(data.start_time)
        .bind(data.expire_time)
        .bind(data.user_limit)
        .bind(&data.subscribe)
        .bind(data.used_count)
        .bind(data.enable)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM coupon WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn update_count(&self, code: &str) -> Result<(), sqlx::Error> {
        let coupon = self.find_one_by_code(code).await?;
        let mut updated = coupon.clone();
        updated.used_count += 1;
        self.update(&updated).await?;
        Ok(())
    }

    async fn query_list_by_page(
        &self,
        page: i64,
        size: i64,
        subscribe: Option<i64>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Coupon>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        let mut idx = 0u32;
        if subscribe.is_some() {
            idx += 1;
            clauses.push(format!("subscribe ILIKE ${}", idx));
        }
        if search.is_some() {
            idx += 1;
            clauses.push(format!("(name ILIKE ${} OR code ILIKE ${})", idx, idx));
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM coupon {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(v) = subscribe {
            count_q = count_q.bind(format!("%{}%", v));
        }
        if let Some(s) = search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM coupon {} ORDER BY id DESC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, Coupon>(audit(&list_sql));
        if let Some(v) = subscribe {
            list_q = list_q.bind(format!("%{}%", v));
        }
        if let Some(s) = search {
            list_q = list_q.bind(format!("%{}%", s));
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn batch_delete(&self, ids: &[i64]) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("${}", i)).collect();
        let sql = format!("DELETE FROM coupon WHERE id IN ({})", placeholders.join(", "));
        let mut q = sqlx::query(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected())
    }
}
