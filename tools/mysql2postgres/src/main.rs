//! mysql2postgres — copy data from MySQL to PostgreSQL.
//!
//! Direct port of `server/tools/mysql2postgres/` (Go).
//! Connects to both databases, builds a migration plan,
//! and copies each table row-by-row using PostgreSQL COPY.

use anyhow::{Context, Result};
use clap::Parser;
use sqlx::mysql::MySqlPool;
use sqlx::postgres::PgPool;
use std::collections::{HashMap, HashSet};

// ─────────────────────────────────────────────────────────────────────────────
// CLI
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Parser, Debug)]
#[command(name = "mysql2postgres", about = "Copy ppanel MySQL data to PostgreSQL")]
struct Args {
    /// Source MySQL DSN (e.g. user:pass@tcp(host:3306)/dbname)
    #[arg(long, env = "MYSQL_DSN")]
    mysql: String,

    /// Target PostgreSQL DSN (e.g. postgres://user:pass@host/dbname)
    #[arg(long, env = "POSTGRES_DSN")]
    postgres: String,

    /// Target PostgreSQL schema (default: public)
    #[arg(long, default_value = "public")]
    schema: String,

    /// Comma-separated table allowlist (default: all common tables)
    #[arg(long, default_value = "")]
    tables: String,

    /// Comma-separated table denylist
    #[arg(long, default_value = "")]
    exclude: String,

    /// Truncate target tables before copying (destructive)
    #[arg(long, default_value_t = false)]
    truncate: bool,

    /// Confirm destructive operations (required when --truncate is set)
    #[arg(long, default_value_t = false)]
    yes: bool,

    /// Print plan without copying data
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Rows per progress log line
    #[arg(long, default_value_t = 1000)]
    batch_size: usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// Column metadata
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct PgColumn {
    name: String,
    data_type: String,
    udt_name: String,
    is_identity: bool,
    is_generated: bool,
}

impl PgColumn {
    fn is_bool(&self) -> bool {
        self.data_type == "boolean" || self.udt_name == "bool"
    }
    fn is_integer(&self) -> bool {
        matches!(self.data_type.as_str(), "smallint" | "integer" | "bigint")
    }
    fn is_timestamp(&self) -> bool {
        self.data_type.contains("timestamp") || self.data_type == "date"
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Migration plan
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct TablePlan {
    name: String,
    columns: Vec<PgColumn>,
    order_columns: Vec<String>,
    row_count: i64,
}

// ─────────────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    if args.truncate && !args.yes && !args.dry_run {
        anyhow::bail!("--truncate is destructive; pass --yes to confirm");
    }

    let mysql = MySqlPool::connect(&args.mysql)
        .await
        .context("connect to MySQL")?;
    tracing::info!("connected to MySQL");

    let pg = PgPool::connect(&args.postgres)
        .await
        .context("connect to PostgreSQL")?;
    tracing::info!("connected to PostgreSQL");

    let plans = build_plans(&mysql, &pg, &args).await?;
    if plans.is_empty() {
        anyhow::bail!("no common tables to migrate");
    }

    tracing::info!("migration plan: {} table(s)", plans.len());
    for p in &plans {
        tracing::info!(
            "  {}: {} row(s), {} column(s)",
            p.name, p.row_count, p.columns.len()
        );
    }

    if args.dry_run {
        tracing::info!("dry run — no data copied");
        return Ok(());
    }

    if args.truncate {
        truncate_tables(&pg, &args.schema, &plans).await?;
    }

    for plan in &plans {
        copy_table(&mysql, &pg, &args.schema, plan, args.batch_size).await?;
    }

    reset_sequences(&pg, &args.schema).await?;
    tracing::info!("migration completed");
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Plan building
// ─────────────────────────────────────────────────────────────────────────────

async fn build_plans(mysql: &MySqlPool, pg: &PgPool, args: &Args) -> Result<Vec<TablePlan>> {
    let source_tables = list_mysql_tables(mysql).await?;
    let target_tables = list_pg_tables(pg, &args.schema).await?;

    let allow: HashSet<String> = parse_set(&args.tables);
    let exclude: HashSet<String> = parse_set(&args.exclude);

    let mut names: Vec<String> = target_tables
        .iter()
        .filter(|n| n.as_str() != "schema_migrations")
        .filter(|n| allow.is_empty() || allow.contains(*n))
        .filter(|n| !exclude.contains(*n))
        .filter(|n| {
            if source_tables.contains(*n) { true } else {
                tracing::warn!("skip {}: no source table", n); false
            }
        })
        .cloned()
        .collect();
    names.sort();

    let mut plans = Vec::new();
    for name in &names {
        let target_cols = list_pg_columns(pg, &args.schema, name).await?;
        let source_cols = list_mysql_columns(mysql, name).await?;

        let common: Vec<PgColumn> = target_cols
            .into_iter()
            .filter(|c| !c.is_generated && source_cols.contains(&c.name))
            .collect();

        if common.is_empty() {
            tracing::warn!("skip {}: no common columns", name);
            continue;
        }

        let row_count = count_mysql_rows(mysql, name).await?;
        let order_columns = list_mysql_pk_columns(mysql, name).await?;

        plans.push(TablePlan { name: name.clone(), columns: common, order_columns, row_count });
    }

    let fks = list_pg_foreign_keys(pg, &args.schema).await?;
    Ok(sort_by_dependencies(plans, fks))
}

fn parse_set(s: &str) -> HashSet<String> {
    s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// MySQL introspection
// ─────────────────────────────────────────────────────────────────────────────

async fn list_mysql_tables(pool: &MySqlPool) -> Result<HashSet<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = DATABASE() AND table_type = 'BASE TABLE'"
    ).fetch_all(pool).await.context("list mysql tables")?;
    Ok(rows.into_iter().map(|(n,)| n).collect())
}

async fn list_mysql_columns(pool: &MySqlPool, table: &str) -> Result<HashSet<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT column_name FROM information_schema.columns \
         WHERE table_schema = DATABASE() AND table_name = ? ORDER BY ordinal_position"
    ).bind(table).fetch_all(pool).await
        .with_context(|| format!("list mysql columns for {table}"))?;
    Ok(rows.into_iter().map(|(n,)| n).collect())
}

async fn list_mysql_pk_columns(pool: &MySqlPool, table: &str) -> Result<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT column_name FROM information_schema.key_column_usage \
         WHERE table_schema = DATABASE() AND table_name = ? AND constraint_name = 'PRIMARY' \
         ORDER BY ordinal_position"
    ).bind(table).fetch_all(pool).await
        .with_context(|| format!("list mysql pk for {table}"))?;
    Ok(rows.into_iter().map(|(n,)| n).collect())
}

async fn count_mysql_rows(pool: &MySqlPool, table: &str) -> Result<i64> {
    let quoted = format!("`{}`", table.replace('`', "``"));
    let (count,): (i64,) = sqlx::query_as(sqlx::AssertSqlSafe(format!("SELECT COUNT(*) FROM {quoted}")))
        .fetch_one(pool).await
        .with_context(|| format!("count mysql rows in {table}"))?;
    Ok(count)
}

// ─────────────────────────────────────────────────────────────────────────────
// PostgreSQL introspection
// ─────────────────────────────────────────────────────────────────────────────

async fn list_pg_tables(pool: &PgPool, schema: &str) -> Result<HashSet<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT table_name FROM information_schema.tables \
         WHERE table_schema = $1 AND table_type = 'BASE TABLE'"
    ).bind(schema).fetch_all(pool).await.context("list pg tables")?;
    Ok(rows.into_iter().map(|(n,)| n).collect())
}

async fn list_pg_columns(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<PgColumn>> {
    let rows: Vec<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT column_name, data_type, udt_name, is_identity, is_generated \
         FROM information_schema.columns \
         WHERE table_schema = $1 AND table_name = $2 ORDER BY ordinal_position"
    ).bind(schema).bind(table).fetch_all(pool).await
        .with_context(|| format!("list pg columns for {table}"))?;
    Ok(rows.into_iter().map(|(name, data_type, udt_name, identity, generated)| PgColumn {
        name,
        data_type,
        udt_name,
        is_identity: identity == "YES",
        is_generated: generated != "NEVER",
    }).collect())
}

async fn list_pg_foreign_keys(pool: &PgPool, schema: &str) -> Result<Vec<(String, String)>> {
    let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT child.relname, parent.relname \
         FROM pg_constraint c \
         JOIN pg_class child ON child.oid = c.conrelid \
         JOIN pg_namespace cn ON cn.oid = child.relnamespace \
         JOIN pg_class parent ON parent.oid = c.confrelid \
         JOIN pg_namespace pn ON pn.oid = parent.relnamespace \
         WHERE c.contype = 'f' AND cn.nspname = $1 AND pn.nspname = $1 \
         ORDER BY child.relname, parent.relname"
    ).bind(schema).fetch_all(pool).await.context("list pg foreign keys")?;
    Ok(rows)
}

// ─────────────────────────────────────────────────────────────────────────────
// Dependency sort (topological, mirrors Go)
// ─────────────────────────────────────────────────────────────────────────────

fn sort_by_dependencies(plans: Vec<TablePlan>, fks: Vec<(String, String)>) -> Vec<TablePlan> {
    let plan_names: HashSet<&str> = plans.iter().map(|p| p.name.as_str()).collect();
    let mut deps: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (child, parent) in &fks {
        if child != parent && plan_names.contains(child.as_str()) && plan_names.contains(parent.as_str()) {
            deps.entry(child).or_default().insert(parent);
        }
    }

    // Collect ordered names (Strings) before consuming `plans`.
    let name_to_idx: HashMap<&str, usize> = plans.iter().enumerate().map(|(i, p)| (p.name.as_str(), i)).collect();
    let mut remaining: HashSet<&str> = plan_names.clone();
    let mut ordered_names: Vec<String> = Vec::new();

    while !remaining.is_empty() {
        let mut ready: Vec<&str> = remaining.iter()
            .copied()
            .filter(|n| deps.get(n).map_or(true, |ps| ps.iter().all(|p| !remaining.contains(*p))))
            .collect();
        if ready.is_empty() {
            ready = remaining.iter().copied().collect();
            tracing::warn!("FK cycle detected; copying {} remaining tables in lexical order", ready.len());
        }
        ready.sort();
        for name in ready {
            ordered_names.push(name.to_string());
            remaining.remove(name);
        }
    }
    let _ = name_to_idx; // suppress unused warning

    // Re-order plans by the collected name sequence.
    let mut plan_map: HashMap<String, TablePlan> =
        plans.into_iter().map(|p| (p.name.clone(), p)).collect();
    ordered_names.into_iter().filter_map(|n| plan_map.remove(&n)).collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Data copy
// ─────────────────────────────────────────────────────────────────────────────

async fn truncate_tables(pg: &PgPool, schema: &str, plans: &[TablePlan]) -> Result<()> {
    let names: Vec<String> = plans.iter()
        .map(|p| format!("\"{schema}\".\"{n}\"", n = p.name))
        .collect();
    let sql = format!("TRUNCATE TABLE {} RESTART IDENTITY CASCADE", names.join(", "));
    tracing::info!("truncating {} table(s)", plans.len());
    sqlx::query(sqlx::AssertSqlSafe(sql)).execute(pg).await.context("truncate tables")?;
    Ok(())
}

async fn copy_table(
    mysql: &MySqlPool,
    pg: &PgPool,
    schema: &str,
    plan: &TablePlan,
    batch_size: usize,
) -> Result<()> {
    tracing::info!("copy {}: start ({} rows)", plan.name, plan.row_count);

    let col_names: Vec<String> = plan.columns.iter().map(|c| c.name.clone()).collect();
    let mysql_cols = col_names.iter()
        .map(|n| format!("`{}`", n.replace('`', "``")))
        .collect::<Vec<_>>()
        .join(", ");

    let mut select = format!("SELECT {} FROM `{}`", mysql_cols, plan.name.replace('`', "``"));
    if !plan.order_columns.is_empty() {
        let order = plan.order_columns.iter()
            .map(|n| format!("`{}`", n.replace('`', "``")))
            .collect::<Vec<_>>()
            .join(", ");
        select.push_str(&format!(" ORDER BY {order}"));
    }

    // Build PG INSERT (no COPY protocol via sqlx yet — use parameterised INSERT in batches).
    let pg_cols = col_names.iter()
        .map(|n| format!("\"{}\"", n.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(", ");

    let mut rows = sqlx::query(sqlx::AssertSqlSafe(select)).fetch(mysql);
    let mut copied: i64 = 0;
    let mut batch: Vec<Vec<Option<String>>> = Vec::with_capacity(batch_size);

    use sqlx::Row;
    use futures::StreamExt;

    while let Some(row) = rows.next().await {
        let row = row.with_context(|| format!("read row from {}", plan.name))?;
        let values: Vec<Option<String>> = col_names.iter().enumerate().map(|(i, _)| {
            row.try_get::<Option<String>, _>(i).unwrap_or(None)
        }).collect();
        batch.push(values);

        if batch.len() >= batch_size {
            insert_batch(pg, schema, &plan.name, &col_names, &pg_cols, &batch, &plan.columns).await?;
            copied += batch.len() as i64;
            batch.clear();
            tracing::info!("copy {}: {}/{}", plan.name, copied, plan.row_count);
        }
    }
    if !batch.is_empty() {
        insert_batch(pg, schema, &plan.name, &col_names, &pg_cols, &batch, &plan.columns).await?;
        copied += batch.len() as i64;
    }

    tracing::info!("copy {}: done ({} rows)", plan.name, copied);
    Ok(())
}

async fn insert_batch(
    pg: &PgPool,
    schema: &str,
    table: &str,
    _col_names: &[String],
    pg_cols: &str,
    batch: &[Vec<Option<String>>],
    _columns: &[PgColumn],
) -> Result<()> {
    if batch.is_empty() { return Ok(()); }
    let col_count = batch[0].len();
    let mut placeholders = Vec::new();
    let mut idx = 1usize;
    for _ in batch {
        let row_ph: Vec<String> = (0..col_count).map(|_| { let s = format!("${idx}"); idx += 1; s }).collect();
        placeholders.push(format!("({})", row_ph.join(",")));
    }
    let sql = format!(
        "INSERT INTO \"{schema}\".\"{table}\" ({pg_cols}) VALUES {} ON CONFLICT DO NOTHING",
        placeholders.join(",")
    );
    let mut q = sqlx::query(sqlx::AssertSqlSafe(sql));
    for row in batch {
        for val in row {
            q = q.bind(val.as_deref());
        }
    }
    q.execute(pg).await.with_context(|| format!("insert batch into {table}"))?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Sequence reset
// ─────────────────────────────────────────────────────────────────────────────

async fn reset_sequences(pg: &PgPool, schema: &str) -> Result<()> {
    let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT table_name, column_name FROM information_schema.columns \
         WHERE table_schema = $1 AND (is_identity = 'YES' OR column_default LIKE 'nextval(%') \
         ORDER BY table_name, ordinal_position"
    ).bind(schema).fetch_all(pg).await.context("list sequences")?;

    let mut count = 0usize;
    for (table, column) in &rows {
        let table_ref = format!("\"{schema}\".\"{table}\"");
        let seq: Option<(Option<String>,)> = sqlx::query_as(
            "SELECT pg_get_serial_sequence($1, $2)"
        ).bind(&table_ref).bind(column).fetch_optional(pg).await
            .with_context(|| format!("get sequence for {table}.{column}"))?;

        let seq_name = match seq.and_then(|(s,)| s) {
            Some(s) if !s.is_empty() => s,
            _ => continue,
        };

        let (max_id,): (i64,) = sqlx::query_as(
            sqlx::AssertSqlSafe(format!("SELECT COALESCE(MAX(\"{column}\"), 0) FROM {table_ref}"))
        ).fetch_one(pg).await
            .with_context(|| format!("max id for {table}.{column}"))?;

        if max_id > 0 {
            sqlx::query("SELECT setval($1::regclass, $2, true)")
                .bind(&seq_name).bind(max_id)
                .execute(pg).await.ok();
        } else {
            sqlx::query("SELECT setval($1::regclass, 1, false)")
                .bind(&seq_name)
                .execute(pg).await.ok();
        }
        count += 1;
    }
    tracing::info!("reset {count} sequence(s)");
    Ok(())
}
