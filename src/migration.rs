//! Database bootstrap utilities.
//!
//! Schema migrations are owned by the Go static library linked in via
//! the `migrate` crate (which uses `rust2go` FFI). The Go code embeds
//! the same SQL migration files and applies them via `golang-migrate`,
//! tracking state in the `schema_migrations` table — the same table
//! the original Go server used. That makes a single source of truth
//! for schema across the Go and Rust backends.
//!
//! On startup, this module invokes the FFI-bound `migrate::up` call.
//! The Go-side logic is idempotent — if the DB is already at the
//! latest version it returns success with no side effects.
//!
//! **NOTE**: This is the Rust rewrite of the Go backend. The Go
//! version is deprecated and will be replaced by this Rust
//! implementation. The Go migration code is retained only as an
//! in-process FFI dependency, not as a separately-spawned binary.

pub use crate::repository::Dialect;

use crate::config::DatabaseConfig;
use crate::db::build_dsn;
use crate::repository::Db;

/// Apply all pending schema migrations via the rust2go FFI bridge.
///
/// Always called on startup — the migrator is idempotent and a no-op
/// if the database is already at the latest version. Failures are
/// fatal: the Go side reports the error in `MigrateOutcome.error` and
/// we `panic!` with that message, aborting startup. This matches the
/// fail-fast behaviour of the previous subprocess-based
/// implementation.
pub async fn ensure_schema(cfg: &DatabaseConfig) {
    let driver = match cfg.driver.as_str() {
        "mysql" => "mysql",
        _ => "postgres",
    };
    let dsn = build_dsn(cfg);

    tracing::info!(driver, "running schema migrations via rust2go FFI");

    // The FFI call is synchronous and may take seconds (open conn,
    // apply all pending migrations, close conn). Run it on a blocking
    // thread so we don't stall the tokio reactor if other startup
    // tasks were racing us.
    let outcome = tokio::task::spawn_blocking(move || {
        migrate::up(driver, &dsn)
    })
    .await
    .expect("migration task panicked");

    tracing::info!(
        version = outcome.version,
        dirty = outcome.dirty,
        "schema migrations complete"
    );
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

    let now_ts = chrono::Utc::now(); // chrono::DateTime<Utc> -> PG timestamp
    let password_hash = hash_password_pbkdf2(password);
    let refer_code = generate_invite_code();

    sqlx::query(
        r#"INSERT INTO "user" (password, algo, is_admin, refer_code, balance, commission,
               gift_amount, enable, enable_balance_notify,
               enable_login_notify, enable_subscribe_notify,
               enable_trade_notify, created_at, updated_at)
           VALUES ($1, 'default', TRUE, $2, 0, 0, 0, TRUE, TRUE, TRUE, TRUE, TRUE, $3, $4)"#,
    )
    .bind(&password_hash)
    .bind(&refer_code)
    .bind(now_ts)
    .bind(now_ts)
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
           VALUES ($1, 'email', $2, TRUE, $3, $4)"#,
    )
    .bind(user_id)
    .bind(email)
    .bind(now_ts)
    .bind(now_ts)
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
