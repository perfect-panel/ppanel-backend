//! Rust binding to the Go migration tool (`tools/migrate/migrate`).
//!
//! This crate wraps the Go-based schema migrator behind a Rust API
//! that can be called directly from the main binary — no subprocess,
//! no out-of-band `ppanel-migrate` binary, no `PPANEL_MIGRATE_BIN`
//! resolution.
//!
//! # Architecture
//!
//! ```text
//! ┌────────────────────────┐       FFI        ┌──────────────────────┐
//! │ ppanel-backend (Rust)  │ ───────────────► │ Go staticlib         │
//! │  - main.rs             │                  │  - golang-migrate    │
//! │  - migration.rs        │                  │  - SQL files         │
//! │                        │                  │  (embedded via       │
//! │                        │                  │   //go:embed)         │
//! └────────────────────────┘                  └──────────────────────┘
//! ```
//!
//! The Go side opens its own short-lived database connection per
//! call. The connection is closed when the call returns. We do not
//! share sqlx pools across the FFI boundary — that would require
//! additional plumbing for no real benefit (the migrator only runs
//! at startup).
//!
//! # Build
//!
//! The build is driven by `build.rs`, which calls
//! `rust2go::Builder`. The Builder compiles the Go code as CGO and
//! produces `_go_bindings.rs` containing the C declarations used by
//! the Rust side. The Go glue (`gen.go`) is regenerated automatically
//! whenever `src/idl.rs` changes.
//!
//! # Runtime
//!
//! The user must set the env vars documented by rust2go when the
//! binary is launched:
//!
//! ```text
//! GODEBUG=invalidptr=0,cgocheck=0
//! ```
//!
//! Without these, Go's conservative GC pointer checks will flag the
//! FFI references as invalid and abort. This is a property of
//! rust2go, not specific to this crate.

pub mod idl;

pub use idl::{MigrateConfig, MigrateOutcome, MigrateService, MigrateServiceImpl};

/// Apply all pending schema migrations.
///
/// Mirrors the behaviour of the previous `ppanel-migrate up`
/// subprocess call:
///   - **Idempotent**: if the database is already at the latest
///     version, returns silently with the current version.
///   - **Fail-fast**: on any Go-side error, panics with the Go error
///     message. The migrator is called at startup before any traffic
///     is served, so failing the process is the correct behaviour.
///
/// # Arguments
///   - `driver`: `"postgres"` or `"mysql"`.
///   - `dsn`: connection string in golang-migrate URL form. The Go
///     side will auto-prepend the URL scheme if missing.
pub fn up(driver: &str, dsn: &str) -> MigrateOutcome {
    let cfg = MigrateConfig {
        driver: driver.to_string(),
        dsn: dsn.to_string(),
    };
    let outcome = MigrateServiceImpl::up(cfg);
    if !outcome.error.is_empty() {
        panic!(
            "ppanel-migrate up failed: {}\n\
             Build via: `cd tools/migrate && go build -o ppanel-migrate ./cmd/migrate`\n\
             (or rely on the rust2go FFI path embedded at build time).",
            outcome.error
        );
    }
    outcome
}

/// Read the current schema version without applying anything.
///
/// Returns `MigrateOutcome { version: 0, dirty: false, error: "" }`
/// for a fresh database (no `schema_migrations` rows).
pub fn version(driver: &str, dsn: &str) -> MigrateOutcome {
    let cfg = MigrateConfig {
        driver: driver.to_string(),
        dsn: dsn.to_string(),
    };
    MigrateServiceImpl::version(cfg)
}
