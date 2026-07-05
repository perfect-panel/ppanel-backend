use crate::model::entity::system::System;
use crate::repository::system::SystemRepo;

pub struct PgSystemRepo {
    pool: sqlx::PgPool,
}

impl PgSystemRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl SystemRepo for PgSystemRepo {
    async fn insert(&self, data: &System) -> Result<System, sqlx::Error> {
        sqlx::query_as::<_, System>(
            r#"INSERT INTO "system" (category, key, value, "type", desc, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               RETURNING *"#,
        )
        .bind(&data.category)
        .bind(&data.key)
        .bind(&data.value)
        .bind(&data.type_)
        .bind(&data.desc)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<System, sqlx::Error> {
        sqlx::query_as::<_, System>(r#"SELECT * FROM "system" WHERE id = $1"#)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &System) -> Result<System, sqlx::Error> {
        sqlx::query_as::<_, System>(
            r#"UPDATE "system" SET category = $1, key = $2, value = $3, "type" = $4, desc = $5, updated_at = $6
               WHERE id = $7 RETURNING *"#,
        )
        .bind(&data.category)
        .bind(&data.key)
        .bind(&data.value)
        .bind(&data.type_)
        .bind(&data.desc)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(r#"DELETE FROM "system" WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn get_by_category(&self, category: &str) -> Result<Vec<System>, sqlx::Error> {
        sqlx::query_as::<_, System>(r#"SELECT * FROM "system" WHERE category = $1"#)
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
            r#"UPDATE "system" SET value = $1 WHERE category = $2 AND key = $3"#,
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
            r#"SELECT * FROM "system" WHERE category = $1 AND key = $2"#,
        )
        .bind(category)
        .bind(key)
        .fetch_one(&self.pool)
        .await
    }
}
