use crate::model::entity::auth::Auth;
use crate::repository::auth::AuthRepo;

pub struct MySqlAuthRepo {
    pool: sqlx::MySqlPool,
}

impl MySqlAuthRepo {
    pub fn new(pool: sqlx::MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuthRepo for MySqlAuthRepo {
    async fn insert(&self, data: &Auth) -> Result<Auth, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO auth_method (method, config, enabled, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&data.method)
        .bind(&data.config)
        .bind(data.enabled)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&self.pool)
        .await?;

        let id = result.last_insert_id() as i64;
        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_one(&self, id: i64) -> Result<Auth, sqlx::Error> {
        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Auth) -> Result<Auth, sqlx::Error> {
        sqlx::query(
            "UPDATE auth_method SET method = ?, config = ?, enabled = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(&data.method)
        .bind(&data.config)
        .bind(data.enabled)
        .bind(data.updated_at)
        .bind(data.id)
        .execute(&self.pool)
        .await?;

        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method WHERE id = ?")
            .bind(data.id)
            .fetch_one(&self.pool)
            .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM auth_method WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(res.rows_affected())
    }

    async fn get_list(&self) -> Result<Vec<Auth>, sqlx::Error> {
        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method")
            .fetch_all(&self.pool)
            .await
    }

    async fn find_one_by_method(&self, method: &str) -> Result<Auth, sqlx::Error> {
        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method WHERE method = ?")
            .bind(method)
            .fetch_one(&self.pool)
            .await
    }

    async fn find_all_enabled(&self) -> Result<Vec<Auth>, sqlx::Error> {
        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method WHERE enabled = true")
            .fetch_all(&self.pool)
            .await
    }
}
