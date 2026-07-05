//! Repository layer — trait-based with per-dialect implementations.
//!
//! Each domain module defines a trait (e.g. `AdsRepo`) and two implementations:
//! `pg::PgXxxRepo` for PostgreSQL and `mysql::MySqlXxxRepo` for MySQL.
//! [`Repositories`] bundles all domain repos behind `Box<dyn>` so handler/service
//! code stays dialect-agnostic.

pub mod ads;
pub mod announcement;
pub mod auth;
pub mod client;
pub mod coupon;
pub mod document;
pub mod log;
pub mod node;
pub mod order;
pub mod payment;
pub mod subscribe;
pub mod system;
pub mod task;
pub mod ticket;
pub mod traffic;
pub mod user;

// ═══════════════════════════════════════════════════════════════════════════
//  Dialect
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    Postgres,
    Mysql,
}

impl Dialect {
    pub fn from_driver(driver: &str) -> Self {
        match driver.to_lowercase().as_str() {
            "postgres" | "postgresql" | "pgsql" => Dialect::Postgres,
            _ => Dialect::Mysql,
        }
    }

    pub fn from_url(url: &str) -> Self {
        if url.starts_with("mysql") {
            Dialect::Mysql
        } else {
            Dialect::Postgres
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  Db — dialect-tagged connection pool
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum Db {
    Postgres(sqlx::Pool<sqlx::Postgres>),
    Mysql(sqlx::Pool<sqlx::MySql>),
}

impl Db {
    pub fn new_pg(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Db::Postgres(pool)
    }

    pub fn new_mysql(pool: sqlx::Pool<sqlx::MySql>) -> Self {
        Db::Mysql(pool)
    }

    pub fn dialect(&self) -> Dialect {
        match self {
            Db::Postgres(_) => Dialect::Postgres,
            Db::Mysql(_) => Dialect::Mysql,
        }
    }

    pub fn pg_pool(&self) -> Option<&sqlx::Pool<sqlx::Postgres>> {
        match self {
            Db::Postgres(p) => Some(p),
            Db::Mysql(_) => None,
        }
    }

    pub fn mysql_pool(&self) -> Option<&sqlx::Pool<sqlx::MySql>> {
        match self {
            Db::Postgres(_) => None,
            Db::Mysql(p) => Some(p),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  Shared helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Mark a dynamically-built SQL string as audited for injection safety.
///
/// sqlx 0.9 gates `query*()` behind the `SqlSafeStr` trait, which is only
/// implemented for `&'static str` out of the box. Every query in this layer
/// is built with `format!` + bound parameters — no user input is ever
/// interpolated into the SQL text — so we wrap each dynamic string in
/// `AssertSqlSafe` here.
#[inline]
pub fn audit(sql: &str) -> sqlx::AssertSqlSafe<&str> {
    sqlx::AssertSqlSafe(sql)
}

/// Paginated result convenience wrapper.
#[derive(Debug, Clone)]
pub struct PageResult<T> {
    pub total: i64,
    pub items: Vec<T>,
}

impl<T> PageResult<T> {
    pub fn new(total: i64, items: Vec<T>) -> Self {
        Self { total, items }
    }
}

/// Normalise a (page, size) pair so the minimum page is 1 and the minimum
/// size is 10.
pub fn normalize_page(page: &mut i64, size: &mut i64) {
    if *page < 1 {
        *page = 1;
    }
    if *size < 1 {
        *size = 10;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
//  Repositories — trait-object bundle
// ═══════════════════════════════════════════════════════════════════════════

/// All domain repositories behind `Box<dyn>` trait objects.
///
/// Constructed once from a [`Db`] via [`Repositories::new`]; the caller
/// (typically `AppState`) holds it and handlers extract individual repos.
pub struct Repositories {
    pub ads: Box<dyn ads::AdsRepo>,
    pub announcement: Box<dyn announcement::AnnouncementRepo>,
    pub auth: Box<dyn auth::AuthRepo>,
    pub client: Box<dyn client::ClientRepo>,
    pub coupon: Box<dyn coupon::CouponRepo>,
    pub document: Box<dyn document::DocumentRepo>,
    pub log: Box<dyn log::LogRepo>,
    pub node: Box<dyn node::NodeRepo>,
    pub order: Box<dyn order::OrderRepo>,
    pub payment: Box<dyn payment::PaymentRepo>,
    pub subscribe: Box<dyn subscribe::SubscribeRepo>,
    pub system: Box<dyn system::SystemRepo>,
    pub task: Box<dyn task::TaskRepo>,
    pub ticket: Box<dyn ticket::TicketRepo>,
    pub traffic: Box<dyn traffic::TrafficRepo>,
    pub user: Box<dyn user::UserRepo>,
}

impl Repositories {
    /// Build the full repository set for the active database dialect.
    pub fn new(db: Db) -> Self {
        match db {
            Db::Postgres(pool) => Self {
                ads: Box::new(ads::pg::PgAdsRepo::new(pool.clone())),
                announcement: Box::new(announcement::pg::PgAnnouncementRepo::new(pool.clone())),
                auth: Box::new(auth::pg::PgAuthRepo::new(pool.clone())),
                client: Box::new(client::pg::PgClientRepo::new(pool.clone())),
                coupon: Box::new(coupon::pg::PgCouponRepo::new(pool.clone())),
                document: Box::new(document::pg::PgDocumentRepo::new(pool.clone())),
                log: Box::new(log::pg::PgLogRepo::new(pool.clone())),
                node: Box::new(node::pg::PgNodeRepo::new(pool.clone())),
                order: Box::new(order::pg::PgOrderRepo::new(pool.clone())),
                payment: Box::new(payment::pg::PgPaymentRepo::new(pool.clone())),
                subscribe: Box::new(subscribe::pg::PgSubscribeRepo::new(pool.clone())),
                system: Box::new(system::pg::PgSystemRepo::new(pool.clone())),
                task: Box::new(task::pg::PgTaskRepo::new(pool.clone())),
                ticket: Box::new(ticket::pg::PgTicketRepo::new(pool.clone())),
                traffic: Box::new(traffic::pg::PgTrafficRepo::new(pool.clone())),
                user: Box::new(user::pg::PgUserRepo::new(pool)),
            },
            Db::Mysql(pool) => Self {
                ads: Box::new(ads::mysql::MySqlAdsRepo::new(pool.clone())),
                announcement: Box::new(announcement::mysql::MySqlAnnouncementRepo::new(pool.clone())),
                auth: Box::new(auth::mysql::MySqlAuthRepo::new(pool.clone())),
                client: Box::new(client::mysql::MySqlClientRepo::new(pool.clone())),
                coupon: Box::new(coupon::mysql::MySqlCouponRepo::new(pool.clone())),
                document: Box::new(document::mysql::MySqlDocumentRepo::new(pool.clone())),
                log: Box::new(log::mysql::MySqlLogRepo::new(pool.clone())),
                node: Box::new(node::mysql::MySqlNodeRepo::new(pool.clone())),
                order: Box::new(order::mysql::MySqlOrderRepo::new(pool.clone())),
                payment: Box::new(payment::mysql::MySqlPaymentRepo::new(pool.clone())),
                subscribe: Box::new(subscribe::mysql::MySqlSubscribeRepo::new(pool.clone())),
                system: Box::new(system::mysql::MySqlSystemRepo::new(pool.clone())),
                task: Box::new(task::mysql::MySqlTaskRepo::new(pool.clone())),
                ticket: Box::new(ticket::mysql::MySqlTicketRepo::new(pool.clone())),
                traffic: Box::new(traffic::mysql::MySqlTrafficRepo::new(pool.clone())),
                user: Box::new(user::mysql::MySqlUserRepo::new(pool)),
            },
        }
    }
}
