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

pub(crate) fn build_dsn(cfg: &DatabaseConfig) -> String {
    let dialect = detect_dialect(cfg);
    let addr = cfg
        .addr
        .as_deref()
        .filter(|a| !a.is_empty())
        .unwrap_or(match dialect {
            Dialect::Postgres => "localhost:5432",
            Dialect::Mysql => "localhost:3306",
        });
    let password = url_encode_password(&cfg.password);
    let query = pick_query(cfg, dialect);

    match dialect {
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

/// Pick the URL query string for the DSN.
///
/// `DatabaseConfig.config` defaults to a MySQL-flavoured string
/// (`charset=utf8mb4&parseTime=true&loc=...`), so an unset YAML field
/// silently gives the wrong params to a Postgres connection. To stay
/// backwards-compatible we only honour `cfg.config` when it looks
/// compatible with the active dialect; otherwise fall back to
/// `default_query`.
fn pick_query(cfg: &DatabaseConfig, dialect: Dialect) -> String {
    let raw = cfg.config.trim();
    if !raw.is_empty() && query_matches_dialect(raw, dialect) {
        return raw.to_string();
    }
    default_query(dialect).to_string()
}

fn query_matches_dialect(query: &str, dialect: Dialect) -> bool {
    // Heuristic — MySQL uses `charset=` / `parseTime=` / `loc=`; Postgres
    // uses `sslmode=` / `TimeZone=` / `channel_binding=`. Treat any MySQL-
    // specific key under a Postgres dialect as a mismatch, and vice versa.
    let q = query.to_ascii_lowercase();
    match dialect {
        Dialect::Postgres => {
            !q.contains("charset=")
                && !q.contains("parsetime=")
                && !q.contains("&loc=")
                && !q.starts_with("loc=")
        }
        Dialect::Mysql => {
            !(q.contains("sslmode=") || q.contains("timezone=") || q.contains("channel_binding="))
        }
    }
}

fn default_query(dialect: Dialect) -> &'static str {
    match dialect {
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
