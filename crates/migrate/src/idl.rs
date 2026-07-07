//! Cross-language interface definition for the migration tool.
//!
//! This file is the **single source of truth** for the Rust↔Go boundary.
//! rust2go reads it and auto-generates:
//!   - `gen.go` (Go-side interface + CGO exports) at
//!     `tools/migrate/migrate/gen.go`
//!   - The C binding file (`_go_bindings.rs` by default) used by the
//!     Rust crate to call into the Go static library.
//!
//! The hand-written Go side lives in `tools/migrate/migrate/impl.go`.

pub mod binding {
    #![allow(warnings)]
    rust2go::r2g_include_binding!();
}

// ═══════════════════════════════════════════════════════════════════════════
//  Wire types — must be `#[derive(R2G, Clone)]` and use only path-local
//  primitive types. See the rust2go skill notes for the supported type
//  matrix.
// ═══════════════════════════════════════════════════════════════════════════

/// Configuration for a single migration invocation. Maps 1:1 to the
/// `-driver` + `-dsn` flags on the standalone CLI.
#[derive(rust2go::R2G, Clone)]
pub struct MigrateConfig {
    /// Either `"postgres"` or `"mysql"`. Any other value causes the Go
    /// side to return an error in `MigrateOutcome.error`.
    pub driver: String,

    /// Full DSN understood by golang-migrate. The Go side will
    /// auto-prepend `postgres://` or `mysql://` if missing, mirroring
    /// the CLI behaviour.
    pub dsn: String,
}

/// Result of a migration call. Errors are signalled by populating
/// `error` with a non-empty message — we deliberately use a plain
/// `String` (not a sum type) so the type marshalling stays trivial and
/// panic-free across the FFI boundary.
#[derive(rust2go::R2G, Clone)]
pub struct MigrateOutcome {
    /// Final schema version after the call. `0` if the
    /// `schema_migrations` table is empty (fresh database).
    pub version: u32,

    /// Whether golang-migrate flagged the migration as dirty. A dirty
    /// state means a previous migration failed mid-way; the caller
    /// should refuse to serve traffic and request human intervention.
    pub dirty: bool,

    /// Empty string on success. Populated with the Go-side error
    /// message on failure. Rust converts this to a panic with the
    /// same semantics as the previous subprocess call.
    pub error: String,
}

// ═══════════════════════════════════════════════════════════════════════════
//  Trait — the cross-language service definition.
// ═══════════════════════════════════════════════════════════════════════════

/// Schema migration service exposed from Go to Rust.
///
/// Each method is a **synchronous** call: Rust blocks the calling
/// thread until Go returns. This is intentional — these are called
/// once at startup before any traffic is served, so we don't need
/// async / shm / drop-safe machinery (which would add complexity for
/// no runtime benefit). The Go side is a single goroutine per call,
/// gated by a mutex on the `init` registration.
#[rust2go::r2g]
pub trait MigrateService {
    /// Apply all pending migrations. Equivalent to `ppanel-migrate up`.
    /// Idempotent: returns success with `version = latest` if the
    /// database is already at the latest source version (matches the
    /// `ErrNoChange` semantics of the standalone CLI).
    fn up(cfg: MigrateConfig) -> MigrateOutcome;

    /// Read the current `schema_migrations.version` and `dirty` flag
    /// without applying anything. Useful for ops/debug.
    fn version(cfg: MigrateConfig) -> MigrateOutcome;
}
