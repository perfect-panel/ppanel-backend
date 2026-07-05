use crate::model::entity::node::{Node, Server, ServerConfigOverride};
use crate::repository::audit;
use crate::repository::node::{NodeFilter, NodeRepo, ServerFilter, SortItem};

pub struct MySqlNodeRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlNodeRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NodeRepo for MySqlNodeRepo {
    async fn insert_server(&self, data: &Server) -> Result<Server, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO servers (name, country, city, address, sort, protocols, last_reported_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.name)
        .bind(&data.country)
        .bind(&data.city)
        .bind(&data.address)
        .bind(data.sort)
        .bind(&data.protocols)
        .bind(data.last_reported_at)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_server(&self, id: i64) -> Result<Server, sqlx::Error> {
        sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_server(&self, data: &Server) -> Result<Server, sqlx::Error> {
        sqlx::query(
            "UPDATE servers SET name = ?, country = ?, city = ?, address = ?,
             sort = ?, protocols = ?, last_reported_at = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.name)
        .bind(&data.country)
        .bind(&data.city)
        .bind(&data.address)
        .bind(data.sort)
        .bind(&data.protocols)
        .bind(data.last_reported_at)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Server>("SELECT * FROM servers WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_server(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM servers WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_node(&self, data: &Node) -> Result<Node, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO nodes (name, tags, port, address, server_id, protocol, enabled, sort, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.name)
        .bind(&data.tags)
        .bind(data.port)
        .bind(&data.address)
        .bind(data.server_id)
        .bind(&data.protocol)
        .bind(data.enabled)
        .bind(data.sort)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_node(&self, id: i64) -> Result<Node, sqlx::Error> {
        sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update_node(&self, data: &Node) -> Result<Node, sqlx::Error> {
        sqlx::query(
            "UPDATE nodes SET name = ?, tags = ?, port = ?, address = ?,
             server_id = ?, protocol = ?, enabled = ?, sort = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.name)
        .bind(&data.tags)
        .bind(data.port)
        .bind(&data.address)
        .bind(data.server_id)
        .bind(&data.protocol)
        .bind(data.enabled)
        .bind(data.sort)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Node>("SELECT * FROM nodes WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_node(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM nodes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn insert_override(&self, data: &ServerConfigOverride) -> Result<ServerConfigOverride, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO server_config_overrides (server_id, ip_strategy, dns, block, outbound, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(data.server_id)
        .bind(&data.ip_strategy)
        .bind(&data.dns)
        .bind(&data.block)
        .bind(&data.outbound)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, ServerConfigOverride>("SELECT * FROM server_config_overrides WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one_override(&self, id: i64) -> Result<ServerConfigOverride, sqlx::Error> {
        sqlx::query_as::<_, ServerConfigOverride>("SELECT * FROM server_config_overrides WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_override_by_server(
        &self,
        server_id: i64,
    ) -> Result<Option<ServerConfigOverride>, sqlx::Error> {
        sqlx::query_as::<_, ServerConfigOverride>(
            "SELECT * FROM server_config_overrides WHERE server_id = ?",
        )
        .bind(server_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn update_override(&self, data: &ServerConfigOverride) -> Result<ServerConfigOverride, sqlx::Error> {
        sqlx::query(
            "UPDATE server_config_overrides SET ip_strategy = ?, dns = ?, block = ?, outbound = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.ip_strategy)
        .bind(&data.dns)
        .bind(&data.block)
        .bind(&data.outbound)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, ServerConfigOverride>("SELECT * FROM server_config_overrides WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete_override(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM server_config_overrides WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn filter_server_list(&self, filter: &ServerFilter) -> Result<(i64, Vec<Server>), sqlx::Error> {
        let mut page = filter.page;
        let mut size = filter.size;
        crate::repository::normalize_page(&mut page, &mut size);
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        if !filter.ids.is_empty() {
            let placeholders: Vec<String> = filter.ids.iter().map(|_| "?".to_string()).collect();
            clauses.push(format!("id IN ({})", placeholders.join(", ")));
        }
        if filter.search.is_some() {
            clauses.push(
                "(LOWER(name) LIKE LOWER(?) OR LOWER(address) LIKE LOWER(?))".to_string(),
            );
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM servers {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        for id in &filter.ids {
            count_q = count_q.bind(id);
        }
        if let Some(s) = &filter.search {
            let pattern = format!("%{}%", s);
            count_q = count_q.bind(pattern.clone()).bind(pattern);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM servers {} ORDER BY sort ASC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Server>(audit(&list_sql));
        for id in &filter.ids {
            list_q = list_q.bind(id);
        }
        if let Some(s) = &filter.search {
            let pattern = format!("%{}%", s);
            list_q = list_q.bind(pattern.clone()).bind(pattern);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn query_server_sorts(&self) -> Result<Vec<SortItem>, sqlx::Error> {
        sqlx::query_as::<_, SortItem>(
            "SELECT id, CAST(sort AS SIGNED) AS sort FROM servers ORDER BY sort ASC",
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn update_server_sort(&self, id: i64, sort: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE servers SET sort = ?, updated_at = UNIX_TIMESTAMP() * 1000 WHERE id = ?",
        )
        .bind(sort)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn count_servers_by_report_status(&self, cutoff: i64) -> Result<(i64, i64), sqlx::Error> {
        let (online,) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM servers WHERE last_reported_at > ?")
            .bind(cutoff)
            .fetch_one(&self.pool)
            .await?;
        let (offline,) = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM servers WHERE last_reported_at <= ? OR last_reported_at IS NULL",
        )
        .bind(cutoff)
        .fetch_one(&self.pool)
        .await?;
        Ok((online, offline))
    }

    async fn query_server_addresses(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows: Vec<(String,)> =
            sqlx::query_as("SELECT address FROM servers ORDER BY id ASC")
                .fetch_all(&self.pool)
                .await?;
        Ok(rows.into_iter().map(|(a,)| a).collect())
    }

    async fn filter_node_list(
        &self,
        filter: &NodeFilter,
        _preload_server: bool,
    ) -> Result<(i64, Vec<Node>), sqlx::Error> {
        let mut page = filter.page;
        let mut size = filter.size;
        crate::repository::normalize_page(&mut page, &mut size);
        let offset = (page - 1) * size;

        let mut clauses = Vec::new();
        if !filter.node_ids.is_empty() {
            let placeholders: Vec<String> = filter.node_ids.iter().map(|_| "?".to_string()).collect();
            clauses.push(format!("id IN ({})", placeholders.join(", ")));
        }
        if !filter.server_ids.is_empty() {
            let placeholders: Vec<String> = filter.server_ids.iter().map(|_| "?".to_string()).collect();
            clauses.push(format!("server_id IN ({})", placeholders.join(", ")));
        }
        if !filter.tags.is_empty() {
            let parts: Vec<String> = filter
                .tags
                .iter()
                .map(|_| "LOWER(tags) LIKE LOWER(?)".to_string())
                .collect();
            clauses.push(format!("({})", parts.join(" OR ")));
        }
        if filter.search.is_some() {
            clauses.push(
                "(LOWER(name) LIKE LOWER(?) OR LOWER(address) LIKE LOWER(?) OR LOWER(tags) LIKE LOWER(?))"
                    .to_string(),
            );
        }
        if filter.protocol.is_some() {
            clauses.push("protocol = ?".to_string());
        }
        if filter.enabled.is_some() {
            clauses.push("enabled = ?".to_string());
        }
        let where_str = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };

        let count_sql = format!("SELECT COUNT(*) FROM nodes {}", where_str);
        let mut count_q = sqlx::query_as::<_, (i64,)>(audit(&count_sql));
        for id in &filter.node_ids {
            count_q = count_q.bind(id);
        }
        for id in &filter.server_ids {
            count_q = count_q.bind(id);
        }
        for t in &filter.tags {
            count_q = count_q.bind(format!("%{}%", t));
        }
        if let Some(s) = &filter.search {
            let pattern = format!("%{}%", s);
            count_q = count_q.bind(pattern.clone()).bind(pattern.clone()).bind(pattern);
        }
        if let Some(p) = &filter.protocol {
            count_q = count_q.bind(p);
        }
        if let Some(e) = filter.enabled {
            count_q = count_q.bind(e);
        }
        let (total,) = count_q.fetch_one(&self.pool).await?;

        let list_sql = format!(
            "SELECT * FROM nodes {} ORDER BY sort ASC LIMIT ? OFFSET ?",
            where_str,
        );
        let mut list_q = sqlx::query_as::<_, Node>(audit(&list_sql));
        for id in &filter.node_ids {
            list_q = list_q.bind(id);
        }
        for id in &filter.server_ids {
            list_q = list_q.bind(id);
        }
        for t in &filter.tags {
            list_q = list_q.bind(format!("%{}%", t));
        }
        if let Some(s) = &filter.search {
            let pattern = format!("%{}%", s);
            list_q = list_q.bind(pattern.clone()).bind(pattern.clone()).bind(pattern);
        }
        if let Some(p) = &filter.protocol {
            list_q = list_q.bind(p);
        }
        if let Some(e) = filter.enabled {
            list_q = list_q.bind(e);
        }
        list_q = list_q.bind(size).bind(offset);
        let items = list_q.fetch_all(&self.pool).await?;

        Ok((total, items))
    }

    async fn query_node_sorts(&self) -> Result<Vec<SortItem>, sqlx::Error> {
        sqlx::query_as::<_, SortItem>(
            "SELECT id, CAST(sort AS SIGNED) AS sort FROM nodes ORDER BY sort ASC",
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn update_node_sort(&self, id: i64, sort: i64) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE nodes SET sort = ? WHERE id = ?")
            .bind(sort)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn query_node_tags(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows: Vec<(String,)> = sqlx::query_as("SELECT tags FROM nodes ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(|(t,)| t).collect())
    }

    async fn count_enabled_nodes(&self) -> Result<i64, sqlx::Error> {
        let (count,) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM nodes WHERE enabled = true")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    async fn query_enabled_node_protocols(&self) -> Result<Vec<String>, sqlx::Error> {
        let rows: Vec<(String,)> =
            sqlx::query_as("SELECT protocol FROM nodes WHERE enabled = true GROUP BY protocol")
                .fetch_all(&self.pool)
                .await?;
        Ok(rows.into_iter().map(|(p,)| p).collect())
    }
}
