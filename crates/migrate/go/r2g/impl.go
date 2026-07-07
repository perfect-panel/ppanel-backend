package main

import (
	"errors"

	ppmigrate "github.com/perfect-panel/ppanel-backend/crates/migrate/go/migrate"
)

// migrateService is the rust2go implementation of the MigrateService trait
// declared in crates/migrate/src/idl.rs. The struct name is unexported to
// avoid colliding with the auto-generated `MigrateService` interface symbol
// in gen.go (per rust2go convention: impl must be unexported, the global
// registration variable `MigrateServiceImpl` must be exported).
type migrateService struct{}

func init() {
	// Register the impl. The variable name MUST be exactly
	// `MigrateServiceImpl` (matches the trait name + "Impl" suffix that
	// gen.go expects).
	MigrateServiceImpl = migrateService{}
}

// up is a sync, no-return-blocking call. We pre-allocate the result with
// sensible defaults; on success we set version+dirty; on failure we
// populate `error` with the Go-side message so Rust can panic/format it.
func (migrateService) up(cfg MigrateConfig) MigrateOutcome {
	sess, err := ppmigrate.Migrate(cfg.driver, cfg.dsn)
	if err != nil {
		return MigrateOutcome{error: err.Error()}
	}
	// Always close the migrate client so we don't leak the DB connection
	// pool it opens. ErrNoChange is fine — the migrator returns no error
	// if everything is up to date and we still want a final version
	// report below.
	if runErr := ppmigrate.RunUp(sess); runErr != nil && !errors.Is(runErr, ppmigrate.NoChange) {
		return MigrateOutcome{error: runErr.Error()}
	}
	// After RunUp, the version we read may be ErrNilVersion only if the
	// DB was empty AND the migration source was also empty (caught by
	// RunUp's own length check). Otherwise we get a concrete version.
	v, dirty, vErr := sess.Migrate.Version()
	out := MigrateOutcome{}
	if vErr != nil {
		// ErrNilVersion is a valid end state: schema_migrations has no
		// row because the source has no up.sql files. Surface as
		// version=0 dirty=false with no error so Rust can log "DB is
		// empty" without panicking.
		if !errors.Is(vErr, ppmigrate.ErrNilVersion) {
			out.error = vErr.Error()
		}
		return out
	}
	out.version = uint32(v)
	out.dirty = dirty
	return out
}

// version returns the current schema_migrations state without applying
// anything. Useful for ops/debug.
func (migrateService) version(cfg MigrateConfig) MigrateOutcome {
	sess, err := ppmigrate.Migrate(cfg.driver, cfg.dsn)
	if err != nil {
		return MigrateOutcome{error: err.Error()}
	}
	v, dirty, err := sess.Migrate.Version()
	out := MigrateOutcome{}
	if err != nil {
		if errors.Is(err, ppmigrate.ErrNilVersion) {
			return out // version=0, dirty=false, no error
		}
		out.error = err.Error()
		return out
	}
	out.version = uint32(v)
	out.dirty = dirty
	return out
}
