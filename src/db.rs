//! Database connection initialisation.
//!
//! Reads the [`DatabaseConfig`] and creates a native `Pool<Postgres>` or
//! `Pool<MySql>` wrapped in [`Db`].  Using concrete pool types (instead of
//! `AnyPool`) lets each repository impl write native SQL with `$N` / `?`
//! placeholders and use `FromRow` against the correct row type.

use std::time::Duration;

use crate::config::DatabaseConfig;
use crate::repository::{Db, Dialect};

/// Build a pool + dialect wrapper from the config subsection.
pub async fn init_pool(cfg: &DatabaseConfig) -> Result<Db, sqlx::Error> {
    let dsn = build_dsn(cfg);
    let dialect = detect_dialect(cfg);

    let max_conn = cfg.max_open_conns.max(1) as u32;

    match dialect {
        Dialect::Postgres => {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(max_conn)
                .acquire_timeout(Duration::from_secs(10))
                .connect(&dsn)
                .await?;
            Ok(Db::new_pg(pool))
        }
        Dialect::Mysql => {
            let pool = sqlx::mysql::MySqlPoolOptions::new()
                .max_connections(max_conn)
                .acquire_timeout(Duration::from_secs(10))
                .connect(&dsn)
                .await?;
            Ok(Db::new_mysql(pool))
        }
    }
}

// ─── DSN builder ─────────────────────────────────────────────────────────

fn build_dsn(cfg: &DatabaseConfig) -> String {
    let addr = cfg
        .addr
        .as_deref()
        .filter(|a| !a.is_empty())
        .unwrap_or(match detect_dialect(cfg) {
            Dialect::Postgres => "localhost:5432",
            Dialect::Mysql => "localhost:3306",
        });
    let password = url_encode_password(&cfg.password);
    let query = if cfg.config.is_empty() {
        default_query(cfg)
    } else {
        &cfg.config
    };

    match detect_dialect(cfg) {
        Dialect::Postgres => format!(
            "postgres://{}:{}@{}/{}?{}",
            cfg.username, password, addr, cfg.dbname, query,
        ),
        Dialect::Mysql => format!(
            "mysql://{}:{}@{}/{}?{}",
            cfg.username, password, addr, cfg.dbname, query,
        ),
    }
}

fn detect_dialect(cfg: &DatabaseConfig) -> Dialect {
    Dialect::from_driver(&cfg.driver)
}

fn default_query(cfg: &DatabaseConfig) -> &'static str {
    match detect_dialect(cfg) {
        Dialect::Postgres => "sslmode=disable&TimeZone=Asia/Shanghai",
        Dialect::Mysql => "charset=utf8mb4&parseTime=true&loc=Asia%2FShanghai",
    }
}

fn url_encode_password(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "%20".into(),
            other => format!("%{:02X}", other as u8),
        })
        .collect()
}
