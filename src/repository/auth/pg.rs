use crate::model::entity::auth::Auth;
use crate::repository::auth::AuthRepo;

pub struct PgAuthRepo {
    pool: sqlx::PgPool,
}

impl PgAuthRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuthRepo for PgAuthRepo {
    async fn insert(&self, data: &Auth) -> Result<Auth, sqlx::Error> {
        sqlx::query_as::<_, Auth>(
            "INSERT INTO auth_method (method, config, enabled, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *",
        )
        .bind(&data.method)
        .bind(&data.config)
        .bind(data.enabled)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_one(&self, id: i64) -> Result<Auth, sqlx::Error> {
        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    async fn update(&self, data: &Auth) -> Result<Auth, sqlx::Error> {
        sqlx::query_as::<_, Auth>(
            "UPDATE auth_method SET method = $1, config = $2, enabled = $3, updated_at = $4
             WHERE id = $5 RETURNING *",
        )
        .bind(&data.method)
        .bind(&data.config)
        .bind(data.enabled)
        .bind(data.updated_at)
        .bind(data.id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query("DELETE FROM auth_method WHERE id = $1")
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
        sqlx::query_as::<_, Auth>("SELECT * FROM auth_method WHERE method = $1")
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
