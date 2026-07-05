use crate::model::entity::client::SubscribeApplication;
use crate::repository::client::ClientRepo;

pub struct MySqlClientRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlClientRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ClientRepo for MySqlClientRepo {
    async fn insert(&self, data: &SubscribeApplication) -> Result<SubscribeApplication, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO subscribe_application (name, icon, description, scheme, user_agent, is_default, subscribe_template, output_format, download_link, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, SubscribeApplication>("SELECT * FROM subscribe_application WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<SubscribeApplication, sqlx::Error> {
        sqlx::query_as::<_, SubscribeApplication>("SELECT * FROM subscribe_application WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &SubscribeApplication) -> Result<SubscribeApplication, sqlx::Error> {
        sqlx::query(
            "UPDATE subscribe_application SET name = ?, icon = ?, description = ?, scheme = ?, user_agent = ?, is_default = ?, subscribe_template = ?, output_format = ?, download_link = ?, updated_at = ?
             WHERE id = ?",
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
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, SubscribeApplication>("SELECT * FROM subscribe_application WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM subscribe_application WHERE id = ?")
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
