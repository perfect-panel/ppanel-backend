package migrate

import (
	"embed"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"

	"github.com/golang-migrate/migrate/v4"
	_ "github.com/golang-migrate/migrate/v4/database/mysql"
	_ "github.com/golang-migrate/migrate/v4/database/postgres"
	"github.com/golang-migrate/migrate/v4/source/iofs"
)

//go:embed sql/postgres/*.sql sql/mysql/*.sql
var sqlFiles embed.FS

// NoChange is re-exported so callers don't have to import golang-migrate directly.
var NoChange = migrate.ErrNoChange

// ErrNilVersion is re-exported for the same reason — it is the sentinel
// returned by Version() when the schema_migrations table is empty.
var ErrNilVersion = migrate.ErrNilVersion

// Session bundles a configured golang-migrate instance alongside the parsed
// source-version list so callers can introspect what migrations exist.
type Session struct {
	Migrate  *migrate.Migrate
	Versions []uint // sorted ascending; all .up.sql versions found in source
}

// Migrate returns a configured golang-migrate instance that reads SQL files from
// the embedded sql/{postgres,mysql}/ directories, based on the requested driver.
//
// driver: "postgres" or "mysql"
// dsn:    golang-migrate URL (e.g. postgres://user:pass@host:port/db?sslmode=disable).
//
// If dsn does not include a URL scheme, the driver is prepended automatically.
//
// Returns an error (no panics) so the function is safe to call across an FFI
// boundary.
func Migrate(driver, dsn string) (*Session, error) {
	sourcePath := "sql/postgres"
	databaseURL := dsn
	switch driver {
	case "mysql":
		sourcePath = "sql/mysql"
		databaseURL = ensureScheme("mysql://", dsn)
	case "postgres":
		databaseURL = ensureScheme("postgres://", dsn)
	default:
		return nil, fmt.Errorf("[Migrate] unsupported database driver: %s", driver)
	}
	d, err := iofs.New(sqlFiles, sourcePath)
	if err != nil {
		return nil, fmt.Errorf("[Migrate] iofs.New error: %v", err)
	}
	client, err := migrate.NewWithSourceInstance("iofs", d, databaseURL)
	if err != nil {
		return nil, fmt.Errorf("[Migrate] NewWithSourceInstance error: %v", err)
	}
	return &Session{
		Migrate:  client,
		Versions: scanVersions(sourcePath),
	}, nil
}

// MigrateOrPanic is a panic-on-error variant retained for the standalone CLI
// (`cmd/migrate/main.go`). Production code (FFI callers) must use Migrate.
func MigrateOrPanic(driver, dsn string) *Session {
	s, err := Migrate(driver, dsn)
	if err != nil {
		panic(err)
	}
	return s
}

// sourceVersionRe matches migration filenames like "02131_xxx.up.sql" / ".down.sql".
var sourceVersionRe = regexp.MustCompile(`^([0-9]+)_[^.]+\.(up|down)\.sql$`)

// scanVersions lists all up-version numbers present in the embedded source dir.
func scanVersions(sourcePath string) []uint {
	entries, err := sqlFiles.ReadDir(sourcePath)
	if err != nil {
		panic(fmt.Errorf("[scanVersions] read %s: %w", sourcePath, err))
	}
	seen := map[uint]struct{}{}
	for _, e := range entries {
		m := sourceVersionRe.FindStringSubmatch(e.Name())
		if m == nil || m[2] != "up" {
			continue
		}
		v, err := strconv.ParseUint(m[1], 10, 64)
		if err != nil {
			continue
		}
		seen[uint(v)] = struct{}{}
	}
	out := make([]uint, 0, len(seen))
	for v := range seen {
		out = append(out, v)
	}
	sort.Slice(out, func(i, j int) bool { return out[i] < out[j] })
	return out
}

// RunUp applies all pending migrations. Unlike m.Up(), it correctly handles the
// "database is already at the latest version" case under the iofs source driver —
// whose Next() / ReadUp() methods return fs.ErrNotExist rather than the
// os.ErrNotExist sentinel that golang-migrate's internal logic checks for.
//
// Returns migrate.ErrNoChange if there is nothing to apply.
func RunUp(s *Session) error {
	if len(s.Versions) == 0 {
		return fmt.Errorf("no migration files embedded")
	}
	srcLast := s.Versions[len(s.Versions)-1]

	dbVer, _, err := s.Migrate.Version()
	if errors.Is(err, migrate.ErrNilVersion) {
		// Empty DB — apply everything from the top.
		return s.Migrate.Up()
	}
	if err != nil {
		return err
	}
	if uint(dbVer) >= srcLast {
		// DB already at or beyond the latest source version. Nothing to do.
		return migrate.ErrNoChange
	}

	steps := int(srcLast - uint(dbVer))
	return s.Migrate.Steps(steps)
}

func ensureScheme(scheme, dsn string) string {
	if strings.Contains(dsn, "://") {
		return dsn
	}
	return scheme + dsn
}

// keep imports referenced (io/fs and os are used elsewhere via errors.Is)
var _ = fs.ErrNotExist
var _ = os.ErrNotExist
