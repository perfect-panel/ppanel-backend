//! End-to-end test for the rust2go FFI migration path.
//!
//! Usage:
//!   GODEBUG=invalidptr=0,cgocheck=0 cargo run --example test_migrate -- \
//!     postgres://user:pass@host:5432/dbname?sslmode=disable
//!
//! Resets the database, runs `migrate::up()` via the FFI bridge, and
//! verifies the resulting schema. Exits with non-zero status on any
//! failure.

use std::env;

fn main() {
    let dsn = env::args()
        .nth(1)
        .expect("usage: test_migrate <postgres-dsn>");

    // Make sure the schema is empty so we can verify migration works
    // from a fresh state. We do this with a side connection in pure
    // SQL — we don't want to import the sqlx stack just for the test.
    // The drop must run as a superuser since `ppanel_test` does not
    // own the public schema.
    println!("== dropping schema public (as postgres superuser) ==");
    let admin_dsn = env::var("PPANEL_ADMIN_DSN")
        .unwrap_or_else(|_| "postgres://postgres@localhost:5432/postgres".to_string());
    let drop_sql = format!(
        "DROP SCHEMA public CASCADE; CREATE SCHEMA public; \
         GRANT ALL ON SCHEMA public TO public; \
         ALTER DATABASE ppanel_rust2go_test OWNER TO ppanel_test;",
    );
    exec_via_psql(&admin_dsn, &drop_sql);

    println!("== calling migrate::up() via FFI ==");
    let outcome = migrate::up("postgres", &dsn);
    println!(
        "  -> version={}, dirty={}, error={:?}",
        outcome.version, outcome.dirty, outcome.error
    );
    if !outcome.error.is_empty() {
        eprintln!("MIGRATION FAILED: {}", outcome.error);
        std::process::exit(1);
    }

    println!("== verifying schema ==");
    let count_sql = "SELECT count(*) FROM information_schema.tables \
                     WHERE table_schema = 'public' AND table_type = 'BASE TABLE';";
    let n: u32 = query_via_psql(&dsn, count_sql)
        .parse()
        .expect("count should parse as u32");
    println!("  -> {} public tables", n);
    if n < 20 {
        eprintln!("expected at least 20 tables, got {}", n);
        std::process::exit(1);
    }

    println!("== verifying schema_migrations state ==");
    let ver_sql = "SELECT version::text || '|' || dirty::text FROM schema_migrations;";
    let state = query_via_psql(&dsn, ver_sql);
    println!("  -> version,dirty = {}", state);
    if !state.starts_with("2131|") {
        eprintln!("expected version 2131, got {}", state);
        std::process::exit(1);
    }

    println!("== testing idempotency: re-run up() ==");
    let outcome2 = migrate::up("postgres", &dsn);
    println!(
        "  -> version={}, dirty={}, error={:?}",
        outcome2.version, outcome2.dirty, outcome2.error
    );
    if !outcome2.error.is_empty() {
        eprintln!("IDEMPOTENT RUN FAILED: {}", outcome2.error);
        std::process::exit(1);
    }

    println!("\n✅ ALL CHECKS PASSED");
}

/// Run a SQL statement via `psql` and return its trimmed stdout.
fn exec_via_psql(dsn: &str, sql: &str) -> String {
    let out = std::process::Command::new("psql")
        .arg(dsn)
        .arg("-c")
        .arg(sql)
        .arg("-A")
        .arg("-t")
        .output()
        .expect("failed to invoke psql (is it installed and on PATH?)");
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        panic!("psql failed: {}\n{}", out.status, stderr);
    }
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn query_via_psql(dsn: &str, sql: &str) -> String {
    exec_via_psql(dsn, sql)
}
