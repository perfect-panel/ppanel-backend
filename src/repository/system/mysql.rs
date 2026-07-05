use crate::model::entity::system::System;
use crate::repository::system::SystemRepo;

pub struct MySqlSystemRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlSystemRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SystemRepo for MySqlSystemRepo {
    async fn insert(&self, data: &System) -> Result<System, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO `system` (category, `key`, value, `type`, `desc`, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&data.category)
        .bind(&data.key)
        .bind(&data.value)
        .bind(&data.type_)
        .bind(&data.desc)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, System>("SELECT * FROM `system` WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<System, sqlx::Error> {
        sqlx::query_as::<_, System>("SELECT * FROM `system` WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &System) -> Result<System, sqlx::Error> {
        sqlx::query(
            "UPDATE `system` SET category = ?, `key` = ?, value = ?, `type` = ?, `desc` = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.category)
        .bind(&data.key)
        .bind(&data.value)
        .bind(&data.type_)
        .bind(&data.desc)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, System>("SELECT * FROM `system` WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM `system` WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn get_by_category(&self, category: &str) -> Result<Vec<System>, sqlx::Error> {
        sqlx::query_as::<_, System>("SELECT * FROM `system` WHERE category = ?")
            .bind(category)
            .fetch_all(&self.pool)
            .await
    }

    async fn update_value_by_category_key(
        &self,
        category: &str,
        key: &str,
        value: &str,
    ) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            "UPDATE `system` SET value = ? WHERE category = ? AND `key` = ?",
        )
        .bind(value)
        .bind(category)
        .bind(key)
        .execute(&self.pool)
        .await?;
        Ok(res.rows_affected())
    }

    async fn find_one_by_category_key(
        &self,
        category: &str,
        key: &str,
    ) -> Result<System, sqlx::Error> {
        sqlx::query_as::<_, System>(
            "SELECT * FROM `system` WHERE category = ? AND `key` = ?",
        )
        .bind(category)
        .bind(key)
        .fetch_one(&self.pool)
        .await
    }
}
