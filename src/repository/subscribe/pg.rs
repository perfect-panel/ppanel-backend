use crate::model::entity::subscribe::{Group, Subscribe};
use crate::repository::audit;
use crate::repository::subscribe::{FilterParams, SubscribeRepo};

pub struct PgSubscribeRepo {
    pool: sqlx::PgPool,
}

impl PgSubscribeRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SubscribeRepo for PgSubscribeRepo {
    async fn insert(&self, data: &Subscribe) -> Result<Subscribe, sqlx::Error> {
        sqlx::query_as::<_, Subscribe>(
            r#"INSERT INTO subscribe (name, language, description, unit_price, unit_time, discount, replacement, inventory, traffic, speed_limit, device_limit, quota, nodes, node_tags, "show", sell, sort, deduction_ratio, allow_deduction, reset_cycle, renewal_reset, show_original_price, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
               RETURNING *"#,
        )
        .bind(&data.name)
        .bind(&data.language)
        .bind(&data.description)
        .bind(data.unit_price)
        .bind(&data.unit_time)
        .bind(&data.discount)
        .bind(data.replacement)
        .bind(data.inventory)
        .bind(data.traffic)
        .bind(data.speed_limit)
        .bind(data.device_limit)
        .bind(data.quota)
        .bind(&data.nodes)
        .bind(&data.node_tags)
        .bind(data.show)
        .bind(data.sell)
        .bind(data.sort)
        .bind(data.deduction_ratio)
        .bind(data.allow_deduction)
        .bind(data.reset_cycle)
        .bind(data.renewal_reset)
        .bind(data.show_original_price)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Subscribe, sqlx::Error> {
        sqlx::query_as::<_, Subscribe>("SELECT * FROM subscribe WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Subscribe) -> Result<Subscribe, sqlx::Error> {
        sqlx::query_as::<_, Subscribe>(
            r#"UPDATE subscribe SET name = $1, language = $2, description = $3, unit_price = $4,
               unit_time = $5, discount = $6, replacement = $7, inventory = $8, traffic = $9,
               speed_limit = $10, device_limit = $11, quota = $12, nodes = $13, node_tags = $14,
               "show" = $15, sell = $16, sort = $17, deduction_ratio = $18, allow_deduction = $19,
               reset_cycle = $20, renewal_reset = $21, show_original_price = $22, updated_at = $23
               WHERE id = $24 RETURNING *"#,
        )
        .bind(&data.name)
        .bind(&data.language)
        .bind(&data.description)
        .bind(data.unit_price)
        .bind(&data.unit_time)
        .bind(&data.discount)
        .bind(data.replacement)
        .bind(data.inventory)
        .bind(data.traffic)
        .bind(data.speed_limit)
        .bind(data.device_limit)
        .bind(data.quota)
        .bind(&data.nodes)
        .bind(&data.node_tags)
        .bind(data.show)
        .bind(data.sell)
        .bind(data.sort)
        .bind(data.deduction_ratio)
        .bind(data.allow_deduction)
        .bind(data.reset_cycle)
        .bind(data.renewal_reset)
        .bind(data.show_original_price)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM subscribe WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn create_group(&self, data: &Group) -> Result<Group, sqlx::Error> {
        sqlx::query_as::<_, Group>(
            r#"INSERT INTO subscribe_group (name, description, created_at, updated_at)
               VALUES ($1, $2, $3, $4)
               RETURNING *"#,
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn update_group(&self, data: &Group) -> Result<Group, sqlx::Error> {
        sqlx::query_as::<_, Group>(
            r#"UPDATE subscribe_group SET name = $1, description = $2, updated_at = $3
               WHERE id = $4 RETURNING *"#,
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_group(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM subscribe_group WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn batch_delete_group(&self, ids: &[i64]) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<String> = ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("${}", i + 1))
            .collect();
        let sql = format!(
            "DELETE FROM subscribe_group WHERE id IN ({})",
            placeholders.join(", ")
        );
        let mut q = sqlx::query(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        let res = q.execute(&self.pool).await?;
        Ok(res.rows_affected())
    }

    async fn query_group_list(&self) -> Result<(i64, Vec<Group>), sqlx::Error> {
        let items =
            sqlx::query_as::<_, Group>("SELECT * FROM subscribe_group ORDER BY id ASC")
                .fetch_all(&self.pool)
                .await?;
        let total = items.len() as i64;
        Ok((total, items))
    }

    async fn update_sort(&self, items: &[Subscribe]) -> Result<(), sqlx::Error> {
        for item in items {
            sqlx::query("UPDATE subscribe SET sort = $1 WHERE id = $2")
                .bind(item.sort)
                .bind(item.id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    async fn query_reset_cycle_subscribe_ids(
        &self,
        reset_cycle: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (i64,)>(
            "SELECT id FROM subscribe WHERE reset_cycle = $1",
        )
        .bind(reset_cycle)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    async fn query_min_sort_by_ids(&self, ids: &[i64]) -> Result<i64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<String> = ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("${}", i + 1))
            .collect();
        let sql = format!(
            "SELECT COALESCE(MIN(sort), 0) FROM subscribe WHERE id IN ({})",
            placeholders.join(", ")
        );
        let mut q = sqlx::query_as::<_, (i64,)>(audit(&sql));
        for id in ids {
            q = q.bind(id);
        }
        let (min_sort,) = q.fetch_one(&self.pool).await?;
        Ok(min_sort)
    }

    async fn filter_list(
        &self,
        params: &mut FilterParams,
    ) -> Result<(i64, Vec<Subscribe>), sqlx::Error> {
        params.normalize();

        let (total, items) = Self::filter_list_lang(&self.pool, params, params.language.as_deref()).await?;

        if total == 0 && params.default_language && params.language.is_some() {
            return Self::filter_list_lang(&self.pool, params, None).await;
        }

        Ok((total, items))
    }
}

impl PgSubscribeRepo {
    async fn filter_list_lang(
        pool: &sqlx::PgPool,
        params: &FilterParams,
        lang: Option<&str>,
    ) -> Result<(i64, Vec<Subscribe>), sqlx::Error> {
        let offset = (params.page - 1) * params.size;

        let mut clauses: Vec<String> = Vec::new();
        let mut idx = 0u32;

        if params.show {
            idx += 1;
            clauses.push(format!("\"show\" = ${}", idx));
        }
        if params.sell {
            idx += 1;
            clauses.push(format!("sell = ${}", idx));
        }
        if params.search.is_some() {
            idx += 1;
            let p = idx;
            clauses.push(format!(
                "(name ILIKE ${} OR description ILIKE ${})",
                p, p
            ));
        }
        if !params.ids.is_empty() {
            let placeholders: Vec<String> = params
                .ids
                .iter()
                .map(|_| {
                    idx += 1;
                    format!("${}", idx)
                })
                .collect();
            clauses.push(format!("id IN ({})", placeholders.join(", ")));
        }
        if !params.nodes.is_empty() {
            let conds: Vec<String> = params
                .nodes
                .iter()
                .map(|_| {
                    idx += 1;
                    format!(
                        "(',' || COALESCE(nodes, '') || ',') LIKE ${}",
                        idx
                    )
                })
                .collect();
            clauses.push(format!("({})", conds.join(" OR ")));
        }
        if !params.tags.is_empty() {
            let conds: Vec<String> = params
                .tags
                .iter()
                .map(|_| {
                    idx += 1;
                    format!(
                        "(',' || COALESCE(node_tags, '') || ',') LIKE ${}",
                        idx
                    )
                })
                .collect();
            clauses.push(format!("({})", conds.join(" OR ")));
        }
        match lang {
            Some(l) if !l.is_empty() => {
                idx += 1;
                clauses.push(format!("language = ${}", idx));
            }
            _ => {
                if params.default_language {
                    clauses.push("(language = '' OR language IS NULL)".to_string());
                }
            }
        }

        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM subscribe {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        if params.show {
            count_q = count_q.bind(true);
        }
        if params.sell {
            count_q = count_q.bind(true);
        }
        if let Some(s) = &params.search {
            count_q = count_q.bind(format!("%{}%", s));
        }
        for id in &params.ids {
            count_q = count_q.bind(id);
        }
        for n in &params.nodes {
            count_q = count_q.bind(format!("%,{},%", n));
        }
        for t in &params.tags {
            count_q = count_q.bind(format!("%,{},%", t));
        }
        if let Some(l) = lang {
            if !l.is_empty() {
                count_q = count_q.bind(l);
            }
        }
        let (total,) = count_q.fetch_one(pool).await?;

        let list_sql = format!(
            "SELECT * FROM subscribe {} ORDER BY sort ASC LIMIT ${} OFFSET ${}",
            where_str,
            idx + 1,
            idx + 2,
        );
        let mut list_q = sqlx::query_as::<_, Subscribe>(audit(&list_sql));
        if params.show {
            list_q = list_q.bind(true);
        }
        if params.sell {
            list_q = list_q.bind(true);
        }
        if let Some(s) = &params.search {
            list_q = list_q.bind(format!("%{}%", s));
        }
        for id in &params.ids {
            list_q = list_q.bind(id);
        }
        for n in &params.nodes {
            list_q = list_q.bind(format!("%,{},%", n));
        }
        for t in &params.tags {
            list_q = list_q.bind(format!("%,{},%", t));
        }
        if let Some(l) = lang {
            if !l.is_empty() {
                list_q = list_q.bind(l);
            }
        }
        list_q = list_q.bind(params.size).bind(offset);
        let items = list_q.fetch_all(pool).await?;

        Ok((total, items))
    }
}
