use crate::model::entity::coupon::Coupon;
use crate::repository::coupon::CouponRepo;
use crate::repository::audit;

pub struct MySqlCouponRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlCouponRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CouponRepo for MySqlCouponRepo {
    async fn insert(&self, data: &Coupon) -> Result<Coupon, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO coupon (name, code, count, `type`, discount, start_time, expire_time, user_limit, subscribe, used_count, enable, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Coupon>("SELECT * FROM coupon WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Coupon, sqlx::Error> {
        sqlx::query_as::<_, Coupon>("SELECT * FROM coupon WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_by_code(&self, code: &str) -> Result<Coupon, sqlx::Error> {
        sqlx::query_as::<_, Coupon>("SELECT * FROM coupon WHERE code = ?")
            .bind(code)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Coupon) -> Result<Coupon, sqlx::Error> {
        sqlx::query(
            "UPDATE coupon SET name = ?, code = ?, count = ?, `type` = ?, discount = ?,
             start_time = ?, expire_time = ?, user_limit = ?, subscribe = ?, used_count = ?,
             enable = ?, updated_at = ?
             WHERE id = ?",
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
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Coupon>("SELECT * FROM coupon WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM coupon WHERE id = ?")
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
        if subscribe.is_some() {
            clauses.push("LOWER(subscribe) LIKE LOWER(?)".to_string());
        }
        if search.is_some() {
            clauses.push("(LOWER(name) LIKE LOWER(?) OR LOWER(code) LIKE LOWER(?))".to_string());
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let subscribe_pattern = subscribe.map(|v| format!("%{}%", v));
        let search_pattern = search.map(|s| format!("%{}%", s));

        let count_sql = format!("SELECT COUNT(*) FROM coupon {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if let Some(ref p) = subscribe_pattern {
            count_q = count_q.bind(p);
        }
        if let Some(ref p) = search_pattern {
            count_q = count_q.bind(p).bind(p);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM coupon {} ORDER BY id DESC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Coupon>(audit(&list_sql));
        if let Some(ref p) = subscribe_pattern {
            list_q = list_q.bind(p);
        }
        if let Some(ref p) = search_pattern {
            list_q = list_q.bind(p).bind(p);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn batch_delete(&self, ids: &[i64]) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM coupon WHERE id IN ({})", placeholders.join(", "));
        let mut q = sqlx::query(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected())
    }
}
