use crate::model::entity::subscribe::{Group, Subscribe};
use crate::repository::audit;
use crate::repository::subscribe::{FilterParams, SubscribeRepo};

pub struct MySqlSubscribeRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlSubscribeRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SubscribeRepo for MySqlSubscribeRepo {
    async fn insert(&self, data: &Subscribe) -> Result<Subscribe, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO subscribe (name, language, description, unit_price, unit_time, discount, replacement, inventory, traffic, speed_limit, device_limit, quota, nodes, node_tags, `show`, sell, sort, deduction_ratio, allow_deduction, reset_cycle, renewal_reset, show_original_price, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Subscribe>("SELECT * FROM subscribe WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Subscribe, sqlx::Error> {
        sqlx::query_as::<_, Subscribe>("SELECT * FROM subscribe WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Subscribe) -> Result<Subscribe, sqlx::Error> {
        sqlx::query(
            "UPDATE subscribe SET name = ?, language = ?, description = ?, unit_price = ?,
             unit_time = ?, discount = ?, replacement = ?, inventory = ?, traffic = ?,
             speed_limit = ?, device_limit = ?, quota = ?, nodes = ?, node_tags = ?,
             `show` = ?, sell = ?, sort = ?, deduction_ratio = ?, allow_deduction = ?,
             reset_cycle = ?, renewal_reset = ?, show_original_price = ?, updated_at = ?
             WHERE id = ?",
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
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Subscribe>("SELECT * FROM subscribe WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM subscribe WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn create_group(&self, data: &Group) -> Result<Group, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO subscribe_group (name, description, created_at, updated_at)
             VALUES (?, ?, ?, ?)",
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Group>("SELECT * FROM subscribe_group WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_group(&self, data: &Group) -> Result<Group, sqlx::Error> {
        sqlx::query(
            "UPDATE subscribe_group SET name = ?, description = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Group>("SELECT * FROM subscribe_group WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_group(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM subscribe_group WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn batch_delete_group(&self, ids: &[i64]) -> Result<u64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
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
            sqlx::query("UPDATE subscribe SET sort = ? WHERE id = ?")
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
        let rows = sqlx::query_as::<_, (i64,)>("SELECT id FROM subscribe WHERE reset_cycle = ?")
            .bind(reset_cycle)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(|(id,)| id).collect())
    }

    async fn query_min_sort_by_ids(&self, ids: &[i64]) -> Result<i64, sqlx::Error> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
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

        let (total, items) =
            Self::filter_list_lang(&self.pool, params, params.language.as_deref()).await?;

        if total == 0 && params.default_language && params.language.is_some() {
            return Self::filter_list_lang(&self.pool, params, None).await;
        }

        Ok((total, items))
    }
}

impl MySqlSubscribeRepo {
    async fn filter_list_lang(
        pool: &sqlx::MySqlPool,
        params: &FilterParams,
        lang: Option<&str>,
    ) -> Result<(i64, Vec<Subscribe>), sqlx::Error> {
        let offset = (params.page - 1) * params.size;

        let mut clauses: Vec<String> = Vec::new();

        if params.show {
            clauses.push("`show` = ?".to_string());
        }
        if params.sell {
            clauses.push("sell = ?".to_string());
        }
        if params.search.is_some() {
            clauses.push(
                "(LOWER(name) LIKE LOWER(?) OR LOWER(description) LIKE LOWER(?))".to_string(),
            );
        }
        if !params.ids.is_empty() {
            let placeholders: Vec<String> = params.ids.iter().map(|_| "?".to_string()).collect();
            clauses.push(format!("id IN ({})", placeholders.join(", ")));
        }
        if !params.nodes.is_empty() {
            let conds: Vec<String> = params
                .nodes
                .iter()
                .map(|_| "FIND_IN_SET(?, nodes)".to_string())
                .collect();
            clauses.push(format!("({})", conds.join(" OR ")));
        }
        if !params.tags.is_empty() {
            let conds: Vec<String> = params
                .tags
                .iter()
                .map(|_| "FIND_IN_SET(?, node_tags)".to_string())
                .collect();
            clauses.push(format!("({})", conds.join(" OR ")));
        }
        match lang {
            Some(l) if !l.is_empty() => {
                clauses.push("language = ?".to_string());
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
            let pattern = format!("%{}%", s);
            let pattern2 = pattern.clone();
            count_q = count_q.bind(pattern).bind(pattern2);
        }
        for id in &params.ids {
            count_q = count_q.bind(id);
        }
        for n in &params.nodes {
            count_q = count_q.bind(n);
        }
        for t in &params.tags {
            count_q = count_q.bind(t);
        }
        if let Some(l) = lang {
            if !l.is_empty() {
                count_q = count_q.bind(l);
            }
        }
        let (total,) = count_q.fetch_one(pool).await?;

        let list_sql = format!(
            "SELECT * FROM subscribe {} ORDER BY sort ASC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Subscribe>(audit(&list_sql));
        if params.show {
            list_q = list_q.bind(true);
        }
        if params.sell {
            list_q = list_q.bind(true);
        }
        if let Some(s) = &params.search {
            let pattern = format!("%{}%", s);
            let pattern2 = pattern.clone();
            list_q = list_q.bind(pattern).bind(pattern2);
        }
        for id in &params.ids {
            list_q = list_q.bind(id);
        }
        for n in &params.nodes {
            list_q = list_q.bind(n);
        }
        for t in &params.tags {
            list_q = list_q.bind(t);
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
