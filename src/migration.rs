//! Database bootstrap utilities.
//!
//! Schema migrations are **not** owned by this Rust binary. They are managed by
//! the standalone Go tool at `tools/migrate/` (built with `go build -o ppanel-migrate ./cmd/migrate`),
//! which uses golang-migrate and tracks state in the `schema_migrations` table
//! — the same table the Go server uses. That makes a single source of truth
//! for schema across the Go and Rust backends.
//!
//! On startup, this module unconditionally invokes the `ppanel-migrate up` command.
//! The tool's own logic is idempotent — if the DB is already at the latest
//! version it returns ErrNoChange with no side effects.
//!
//! **NOTE**: This is the Rust rewrite of the Go backend. The Go version is
//! deprecated and will be replaced by this Rust implementation.

pub use crate::repository::Dialect;

use crate::config::DatabaseConfig;
use crate::db::build_dsn;
use crate::repository::Db;
use std::path::PathBuf;
use std::process::Command;

/// Run all pending schema migrations by invoking the `ppanel-migrate up` command.
/// Always called on startup — the tool is idempotent and is a no-op if the DB
/// is already at the latest version.
pub async fn ensure_schema(cfg: &DatabaseConfig) {
    tracing::info!("running ppanel-migrate up");
    run_migrate_tool(cfg).await;
    tracing::info!("ppanel-migrate completed");
}

/// Resolve the path to the `ppanel-migrate` binary.
///
/// Search order:
/// 1. `${PPANEL_MIGRATE_BIN}` if set.
/// 2. Next to the current executable (`./ppanel-migrate`).
/// 3. One level up from the executable (cargo puts the binary in target/release/;
///    the migrate tool often lives at tools/migrate/ppanel-migrate).
/// 4. `$PATH` (via Command's default lookup).
fn locate_migrate_bin() -> PathBuf {
    if let Ok(p) = std::env::var("PPANEL_MIGRATE_BIN") {
        return PathBuf::from(p);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let candidate = dir.join("ppanel-migrate");
            if candidate.is_file() {
                return candidate;
            }
            if let Some(parent) = dir.parent() {
                let candidate = parent.join("ppanel-migrate");
                if candidate.is_file() {
                    return candidate;
                }
            }
        }
    }
    PathBuf::from("ppanel-migrate")
}

async fn run_migrate_tool(cfg: &DatabaseConfig) {
    let bin = locate_migrate_bin();
    let driver = match cfg.driver.as_str() {
        "mysql" => "mysql",
        _ => "postgres",
    };
    let dsn = build_dsn(cfg);

    tracing::info!(driver, bin = %bin.display(), "running ppanel-migrate up");

    // Run the migrate tool synchronously — it must complete before we serve
    // any traffic. Failures are fatal.
    let output = Command::new(&bin)
        .arg("-driver")
        .arg(driver)
        .arg("-dsn")
        .arg(&dsn)
        .arg("up")
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "failed to execute ppanel-migrate at {}: {e}. \
                 Build it with `go build -o ppanel-migrate ./cmd/migrate` \
                 from ppanel-backend/tools/migrate, or set $PPANEL_MIGRATE_BIN",
                bin.display(),
            )
        });

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!(
            "ppanel-migrate exited with status {}: {}{}{}",
            output.status,
            stdout,
            if !stdout.is_empty() && !stderr.is_empty() { "\n" } else { "" },
            stderr,
        );
    }
    tracing::info!("ppanel-migrate completed");
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