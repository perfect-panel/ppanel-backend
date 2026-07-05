use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::model::entity::order::{Order, OrdersTotal};
use crate::repository::audit;
use crate::repository::order::{OrderDetails, OrderRepo, OrdersTotalWithDate};

pub struct PgOrderRepo {
    pool: sqlx::PgPool,
}

impl PgOrderRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

fn day_start(ts: i64) -> i64 {
    let secs = ts / 1000;
    (secs - secs % 86400) * 1000
}

fn day_end(ts: i64) -> i64 {
    day_start(ts) + 86400 * 1000 - 1
}

fn month_start(ts: i64) -> i64 {
    let dt = DateTime::<Utc>::from_timestamp_millis(ts).unwrap_or_default();
    dt.with_day(1).unwrap()
        .with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
        .timestamp_millis()
}

fn month_end(ts: i64) -> i64 {
    let dt = DateTime::<Utc>::from_timestamp_millis(ts).unwrap_or_default();
    let next = if dt.month() == 12 {
        dt.with_year(dt.year() + 1).unwrap().with_month(1).unwrap()
    } else {
        dt.with_month(dt.month() + 1).unwrap()
    };
    next.with_day(1).unwrap()
        .with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
        .timestamp_millis()
        - 1
}

fn month_start_months_ago(ts: i64, months_ago: i32) -> i64 {
    let dt = DateTime::<Utc>::from_timestamp_millis(ts).unwrap_or_default();
    let total = dt.year() * 12 + (dt.month() as i32) - 1 - months_ago;
    let year = total.div_euclid(12);
    let month = total.rem_euclid(12) + 1;
    dt.with_year(year).unwrap()
        .with_month(month as u32).unwrap()
        .with_day(1).unwrap()
        .with_hour(0).unwrap()
        .with_minute(0).unwrap()
        .with_second(0).unwrap()
        .with_nanosecond(0).unwrap()
        .timestamp_millis()
}

const DETAILS_SELECT: &str = r#"SELECT o.*, s.name AS subscribe_name, p.name AS payment_name
FROM "order" o
LEFT JOIN subscribe s ON o.subscribe_id = s.id
LEFT JOIN payment p ON o.payment_id = p.id"#;

#[async_trait::async_trait]
impl OrderRepo for PgOrderRepo {
    async fn insert(&self, data: &Order) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"INSERT INTO "order" (parent_id, user_id, order_no, "type", quantity, price, amount, gift_amount, discount, coupon, coupon_discount, commission, payment_id, method, fee_amount, trade_no, status, subscribe_id, subscribe_token, is_new, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
               RETURNING *"#,
        )
        .bind(data.parent_id)
        .bind(data.user_id)
        .bind(&data.order_no)
        .bind(data.type_)
        .bind(data.quantity)
        .bind(data.price)
        .bind(data.amount)
        .bind(data.gift_amount)
        .bind(data.discount)
        .bind(&data.coupon)
        .bind(data.coupon_discount)
        .bind(data.commission)
        .bind(data.payment_id)
        .bind(&data.method)
        .bind(data.fee_amount)
        .bind(&data.trade_no)
        .bind(data.status)
        .bind(data.subscribe_id)
        .bind(&data.subscribe_token)
        .bind(data.is_new)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(r#"SELECT * FROM "order" WHERE id = $1"#)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_by_order_no(&self, order_no: &str) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(r#"SELECT * FROM "order" WHERE order_no = $1"#)
            .bind(order_no)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_details(&self, id: i64) -> Result<OrderDetails, sqlx::Error> {
        let sql = format!("{} WHERE o.id = $1", DETAILS_SELECT);
        sqlx::query_as::<_, OrderDetails>(audit(&sql))
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_details_by_order_no(&self, order_no: &str) -> Result<OrderDetails, sqlx::Error> {
        let sql = format!("{} WHERE o.order_no = $1", DETAILS_SELECT);
        sqlx::query_as::<_, OrderDetails>(audit(&sql))
            .bind(order_no)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Order) -> Result<Order, sqlx::Error> {
        sqlx::query_as::<_, Order>(
            r#"UPDATE "order" SET parent_id = $1, user_id = $2, order_no = $3, "type" = $4, quantity = $5, price = $6, amount = $7, gift_amount = $8, discount = $9, coupon = $10, coupon_discount = $11, commission = $12, payment_id = $13, method = $14, fee_amount = $15, trade_no = $16, status = $17, subscribe_id = $18, subscribe_token = $19, is_new = $20, updated_at = $21
               WHERE id = $22 RETURNING *"#,
        )
        .bind(data.parent_id)
        .bind(data.user_id)
        .bind(&data.order_no)
        .bind(data.type_)
        .bind(data.quantity)
        .bind(data.price)
        .bind(data.amount)
        .bind(data.gift_amount)
        .bind(data.discount)
        .bind(&data.coupon)
        .bind(data.coupon_discount)
        .bind(data.commission)
        .bind(data.payment_id)
        .bind(&data.method)
        .bind(data.fee_amount)
        .bind(&data.trade_no)
        .bind(data.status)
        .bind(data.subscribe_id)
        .bind(&data.subscribe_token)
        .bind(data.is_new)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(r#"DELETE FROM "order" WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn update_order_status(&self, order_no: &str, status: i16) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(r#"UPDATE "order" SET status = $1 WHERE order_no = $2"#)
            .bind(status)
            .bind(order_no)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn count_user_coupon_usage(&self, user_id: i64, coupon: &str) -> Result<i64, sqlx::Error> {
        let (count,) =
            sqlx::query_as::<_, (i64,)>(r#"SELECT COUNT(*) FROM "order" WHERE user_id = $1 AND coupon = $2"#)
                .bind(user_id)
                .bind(coupon)
                .fetch_one(&self.pool)
                .await?;
        Ok(count)
    }

    async fn query_list_by_page(
        &self,
        page: i64,
        size: i64,
        status: i16,
        user_id: i64,
        subscribe_id: i64,
        search: Option<&str>,
    ) -> Result<(i64, Vec<OrderDetails>), sqlx::Error> {
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        let mut idx = 0u32;
        if status > 0 {
            idx += 1;
            clauses.push(format!("o.status = ${}", idx));
        }
        if user_id > 0 {
            idx += 1;
            clauses.push(format!("o.user_id = ${}", idx));
        }
        if subscribe_id > 0 {
            idx += 1;
            clauses.push(format!("o.subscribe_id = ${}", idx));
        }
        if search.is_some() {
            idx += 1;
            clauses.push(format!(
                "(o.order_no ILIKE ${} OR o.trade_no ILIKE ${} OR o.coupon ILIKE ${})",
                idx, idx, idx
            ));
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!(r#"SELECT COUNT(*) FROM "order" o {}"#, where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if status > 0 {
            count_q = count_q.bind(status);
        }
        if user_id > 0 {
            count_q = count_q.bind(user_id);
        }
        if subscribe_id > 0 {
            count_q = count_q.bind(subscribe_id);
        }
        if let Some(s) = search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            r#"{} {} ORDER BY o.id DESC LIMIT ${} OFFSET ${}"#,
            DETAILS_SELECT, where_str, idx + 1, idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, OrderDetails>(audit(&list_sql));
        if status > 0 {
            list_q = list_q.bind(status);
        }
        if user_id > 0 {
            list_q = list_q.bind(user_id);
        }
        if subscribe_id > 0 {
            list_q = list_q.bind(subscribe_id);
        }
        if let Some(s) = search {
            list_q = list_q.bind(format!("%{}%", s));
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn is_user_eligible_for_new_order(&self, user_id: i64) -> Result<bool, sqlx::Error> {
        let (count,) = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM "order" WHERE user_id = $1 AND status IN (2, 5)"#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(count == 0)
    }

    async fn query_monthly_orders(&self, now: i64) -> Result<OrdersTotal, sqlx::Error> {
        let start = month_start(now);
        let end = month_end(now);
        sqlx::query_as::<_, OrdersTotal>(
            r#"SELECT
                 COALESCE(SUM(amount), 0) AS amount_total,
                 COALESCE(SUM(CASE WHEN is_new THEN amount ELSE 0 END), 0) AS new_order_amount,
                 COALESCE(SUM(CASE WHEN NOT is_new THEN amount ELSE 0 END), 0) AS renewal_order_amount
               FROM "order"
               WHERE status IN (2, 5) AND created_at >= $1 AND created_at <= $2 AND method <> 'balance'"#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
    }

    async fn query_date_orders(&self, date: i64) -> Result<OrdersTotal, sqlx::Error> {
        let start = day_start(date);
        let end = day_end(date);
        sqlx::query_as::<_, OrdersTotal>(
            r#"SELECT
                 COALESCE(SUM(amount), 0) AS amount_total,
                 COALESCE(SUM(CASE WHEN is_new THEN amount ELSE 0 END), 0) AS new_order_amount,
                 COALESCE(SUM(CASE WHEN NOT is_new THEN amount ELSE 0 END), 0) AS renewal_order_amount
               FROM "order"
               WHERE status IN (2, 5) AND created_at >= $1 AND created_at <= $2 AND method <> 'balance'"#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await
    }

    async fn query_total_orders(&self) -> Result<OrdersTotal, sqlx::Error> {
        sqlx::query_as::<_, OrdersTotal>(
            r#"SELECT
                 COALESCE(SUM(amount), 0) AS amount_total,
                 COALESCE(SUM(CASE WHEN is_new THEN amount ELSE 0 END), 0) AS new_order_amount,
                 COALESCE(SUM(CASE WHEN NOT is_new THEN amount ELSE 0 END), 0) AS renewal_order_amount
               FROM "order"
               WHERE status IN (2, 5) AND method <> 'balance'"#,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn query_daily_orders_list(&self, now: i64) -> Result<Vec<OrdersTotalWithDate>, sqlx::Error> {
        let start = month_start(now);
        let end = month_end(now);
        sqlx::query_as::<_, OrdersTotalWithDate>(
            r#"SELECT
                 TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM-DD') AS date,
                 COALESCE(SUM(amount), 0) AS amount_total,
                 COALESCE(SUM(CASE WHEN is_new THEN amount ELSE 0 END), 0) AS new_order_amount,
                 COALESCE(SUM(CASE WHEN NOT is_new THEN amount ELSE 0 END), 0) AS renewal_order_amount
               FROM "order"
               WHERE status IN (2, 5) AND created_at >= $1 AND created_at <= $2 AND method <> 'balance'
               GROUP BY date
               ORDER BY date ASC"#,
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_monthly_orders_list(&self, now: i64) -> Result<Vec<OrdersTotalWithDate>, sqlx::Error> {
        let start = month_start_months_ago(now, 6);
        let end = month_end(now);
        sqlx::query_as::<_, OrdersTotalWithDate>(
            r#"SELECT
                 TO_CHAR(TO_TIMESTAMP(created_at / 1000), 'YYYY-MM') AS date,
                 COALESCE(SUM(amount), 0) AS amount_total,
                 COALESCE(SUM(CASE WHEN is_new THEN amount ELSE 0 END), 0) AS new_order_amount,
                 COALESCE(SUM(CASE WHEN NOT is_new THEN amount ELSE 0 END), 0) AS renewal_order_amount
               FROM "order"
               WHERE status IN (2, 5) AND created_at >= $1 AND created_at <= $2 AND method <> 'balance'
               GROUP BY date
               ORDER BY date ASC"#,
        )
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await
    }

    async fn query_monthly_user_counts(&self, now: i64) -> Result<(i64, i64), sqlx::Error> {
        let start = month_start(now);
        let end = month_end(now);
        let (new_count, renewal_count) = sqlx::query_as::<_, (i64, i64)>(
            r#"SELECT
                 COUNT(DISTINCT CASE WHEN is_new THEN user_id END) AS new_count,
                 COUNT(DISTINCT CASE WHEN NOT is_new THEN user_id END) AS renewal_count
               FROM "order"
               WHERE status IN (2, 5) AND created_at >= $1 AND created_at <= $2 AND method <> 'balance'"#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;
        Ok((new_count, renewal_count))
    }

    async fn query_date_user_counts(&self, date: i64) -> Result<(i64, i64), sqlx::Error> {
        let start = day_start(date);
        let end = day_end(date);
        let (new_count, renewal_count) = sqlx::query_as::<_, (i64, i64)>(
            r#"SELECT
                 COUNT(DISTINCT CASE WHEN is_new THEN user_id END) AS new_count,
                 COUNT(DISTINCT CASE WHEN NOT is_new THEN user_id END) AS renewal_count
               FROM "order"
               WHERE status IN (2, 5) AND created_at >= $1 AND created_at <= $2 AND method <> 'balance'"#,
        )
        .bind(start)
        .bind(end)
        .fetch_one(&self.pool)
        .await?;
        Ok((new_count, renewal_count))
    }

    async fn query_total_user_counts(&self) -> Result<(i64, i64), sqlx::Error> {
        let (new_count, renewal_count) = sqlx::query_as::<_, (i64, i64)>(
            r#"SELECT
                 COUNT(DISTINCT CASE WHEN is_new THEN user_id END) AS new_count,
                 COUNT(DISTINCT CASE WHEN NOT is_new THEN user_id END) AS renewal_count
               FROM "order"
               WHERE status IN (2, 5) AND method <> 'balance'"#,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok((new_count, renewal_count))
    }
}
