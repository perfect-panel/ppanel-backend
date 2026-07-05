use crate::model::entity::client::SubscribeApplication;
use crate::repository::client::ClientRepo;

pub struct PgClientRepo {
    pool: sqlx::PgPool,
}

impl PgClientRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ClientRepo for PgClientRepo {
    async fn insert(&self, data: &SubscribeApplication) -> Result<SubscribeApplication, sqlx::Error> {
        sqlx::query_as::<_, SubscribeApplication>(
            "INSERT INTO subscribe_application (name, icon, description, scheme, user_agent, is_default, subscribe_template, output_format, download_link, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING *",
        )
        .bind(&data.name)
        .bind(&data.icon)
        .bind(&data.description)
        .bind(&data.scheme)
        .bind(&data.user_agent)
        .bind(data.is_default)
        .bind(&data.subscribe_template)
        .bind(&data.output_format)
        .bind(&data.download_link)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<SubscribeApplication, sqlx::Error> {
        sqlx::query_as::<_, SubscribeApplication>("SELECT * FROM subscribe_application WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &SubscribeApplication) -> Result<SubscribeApplication, sqlx::Error> {
        sqlx::query_as::<_, SubscribeApplication>(
            "UPDATE subscribe_application SET name = $1, icon = $2, description = $3, scheme = $4, user_agent = $5, is_default = $6, subscribe_template = $7, output_format = $8, download_link = $9, updated_at = $10
             WHERE id = $11 RETURNING *",
        )
        .bind(&data.name)
        .bind(&data.icon)
        .bind(&data.description)
        .bind(&data.scheme)
        .bind(&data.user_agent)
        .bind(data.is_default)
        .bind(&data.subscribe_template)
        .bind(&data.output_format)
        .bind(&data.download_link)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM subscribe_application WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn list(&self) -> Result<Vec<SubscribeApplication>, sqlx::Error> {
        sqlx::query_as::<_, SubscribeApplication>("SELECT * FROM subscribe_application ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await
    }
}
