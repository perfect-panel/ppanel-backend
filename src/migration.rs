//! Database migration runner and bootstrap utilities.
//!
//! Embeds both MySQL and PostgreSQL migration files at compile time via
//! `sqlx::migrate!` and selects the correct set at runtime based on the
//! [`Dialect`] detected from configuration.
//!
//! **NOTE**: This is the Rust rewrite of the Go backend. The Go version is
//! deprecated and will be replaced by this Rust implementation.

mod mysql_migrations {
    #![allow(unused)]
    pub(super) const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("migrations/mysql");
}

mod postgres_migrations {
    #![allow(unused)]
    pub(super) const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("migrations/postgres");
}

pub use crate::repository::Dialect;

use crate::repository::Db;

/// Run all pending migrations for the detected database dialect.
pub async fn run_migrations(db: &Db) -> Result<(), sqlx::migrate::MigrateError> {
    match db {
        Db::Postgres(pool) => postgres_migrations::MIGRATOR.run(pool).await,
        Db::Mysql(pool) => mysql_migrations::MIGRATOR.run(pool).await,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  Bootstrap: initial admin account
// ═══════════════════════════════════════════════════════════════════════════

pub async fn create_admin_user(
    db: &Db,
    email: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    match db {
        Db::Postgres(pool) => create_admin_user_pg(pool, email, password).await,
        Db::Mysql(pool) => create_admin_user_mysql(pool, email, password).await,
    }
}

async fn create_admin_user_pg(
    pool: &sqlx::PgPool,
    email: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    let exists: bool =
        sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM "user")"#)
            .fetch_one(pool)
            .await?;

    if exists {
        tracing::info!("User already exists, skip creating administrator account");
        return Ok(());
    }

    let now = chrono::Utc::now().timestamp_millis();
    let password_hash = hash_password_pbkdf2(password);
    let refer_code = generate_invite_code();

    sqlx::query(
        r#"INSERT INTO "user" (password, algo, is_admin, refer_code, balance, commission,
               gift_amount, enable, enable_balance_notify,
               enable_login_notify, enable_subscribe_notify,
               enable_trade_notify, created_at, updated_at)
           VALUES ($1, 'default', 1, $2, 0, 0, 0, 1, 1, 1, 1, 1, $3, $4)"#,
    )
    .bind(&password_hash)
    .bind(&refer_code)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create admin user: {e}");
        e
    })?;

    let user_id: i64 =
        sqlx::query_scalar(r#"SELECT id FROM "user" WHERE refer_code = $1"#)
            .bind(&refer_code)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch admin user id: {e}");
                e
            })?;

    sqlx::query(
        r#"INSERT INTO user_auth_methods (user_id, auth_type, auth_identifier, verified, created_at, updated_at)
           VALUES ($1, 'email', $2, 1, $3, $4)"#,
    )
    .bind(user_id)
    .bind(email)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create admin auth method: {e}");
        e
    })?;

    tracing::info!("Administrator account created: {email}");
    Ok(())
}

async fn create_admin_user_mysql(
    pool: &sqlx::MySqlPool,
    email: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    let exists: bool =
        sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM `user`)"#)
            .fetch_one(pool)
            .await?;

    if exists {
        tracing::info!("User already exists, skip creating administrator account");
        return Ok(());
    }

    let now = chrono::Utc::now().timestamp_millis();
    let password_hash = hash_password_pbkdf2(password);
    let refer_code = generate_invite_code();

    sqlx::query(
        r#"INSERT INTO `user` (password, algo, is_admin, refer_code, balance, commission,
               gift_amount, enable, enable_balance_notify,
               enable_login_notify, enable_subscribe_notify,
               enable_trade_notify, created_at, updated_at)
           VALUES (?, 'default', 1, ?, 0, 0, 0, 1, 1, 1, 1, 1, ?, ?)"#,
    )
    .bind(&password_hash)
    .bind(&refer_code)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create admin user: {e}");
        e
    })?;

    let user_id: i64 =
        sqlx::query_scalar(r#"SELECT id FROM `user` WHERE refer_code = ?"#)
            .bind(&refer_code)
            .fetch_one(pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch admin user id: {e}");
                e
            })?;

    sqlx::query(
        r#"INSERT INTO user_auth_methods (user_id, auth_type, auth_identifier, verified, created_at, updated_at)
           VALUES (?, 'email', ?, 1, ?, ?)"#,
    )
    .bind(user_id)
    .bind(email)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create admin auth method: {e}");
        e
    })?;

    tracing::info!("Administrator account created: {email}");
    Ok(())
}

/// PBKDF2-SHA512 matching Go's format: `$pbkdf2-sha512${salt_hex}${hash_hex}`
///
/// **Iteration count is intentionally 100** to stay byte-compatible with the
/// Go original.  See `AGENTS.md` for rationale.
fn hash_password_pbkdf2(password: &str) -> String {
    password::encode_password(password).expect("Failed to encode password")
}

fn generate_invite_code() -> String {
    format!("u{}", &uuid::Uuid::new_v4().to_string().replace('-', "")[..12])
}

// ═══════════════════════════════════════════════════════════════════════════
//  Bootstrap: initial admin account
// ═══════════════════════════════════════════════════════════════════════════

