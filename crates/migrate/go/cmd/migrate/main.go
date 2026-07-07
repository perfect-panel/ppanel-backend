// ppanel-migrate is a standalone CLI for managing PPanel database schema.
//
// It uses the migrate package (copied from server/initialize/migrate) which embeds
// the SQL migration files and applies them via golang-migrate. Tracks state in the
// schema_migrations table — the same table the Go server uses — so the Rust
// ppanel-backend can attach to a database that was already initialised by either tool.
//
// Usage:
//
//	ppanel-migrate -driver=postgres -dsn="postgres://..." up
//	ppanel-migrate -driver=postgres -dsn="postgres://..." version
//	ppanel-migrate -driver=postgres -dsn="postgres://..." force 2131
package main

import (
	"errors"
	"flag"
	"fmt"
	"log"
	"os"
	"strconv"

	gomigrate "github.com/golang-migrate/migrate/v4"

	ppmigrate "github.com/perfect-panel/ppanel-backend/crates/migrate/go/migrate"
)

func main() {
	var (
		driver  = flag.String("driver", "postgres", "Database driver: postgres | mysql")
		dsn     = flag.String("dsn", "", "Database DSN. URL scheme (postgres:// | mysql://) is auto-prepended if absent.")
		verbose = flag.Bool("v", false, "Verbose logging")
	)
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "ppanel-migrate: standalone PPanel schema migration tool\n\n")
		fmt.Fprintf(os.Stderr, "Usage: ppanel-migrate -driver=<postgres|mysql> -dsn=<dsn> <command> [args]\n\n")
		fmt.Fprintf(os.Stderr, "Commands:\n")
		fmt.Fprintf(os.Stderr, "  up [N]            Apply all (or N) pending migrations\n")
		fmt.Fprintf(os.Stderr, "  down [N]          Roll back one (or N) migration\n")
		fmt.Fprintf(os.Stderr, "  version           Print current schema version\n")
		fmt.Fprintf(os.Stderr, "  force <version>   Mark database at <version> without running migrations\n")
		fmt.Fprintf(os.Stderr, "  drop              Drop every object in the database (dangerous)\n\n")
		flag.PrintDefaults()
	}
	flag.Parse()

	if *dsn == "" {
		flag.Usage()
		os.Exit(2)
	}
	if *verbose {
		log.SetFlags(log.LstdFlags | log.Lmicroseconds)
	}

	cmd := flag.Arg(0)
	sess := ppmigrate.MigrateOrPanic(*driver, *dsn)
	m := sess.Migrate

	switch cmd {
	case "", "up":
		if err := ppmigrate.RunUp(sess); err != nil && !errors.Is(err, gomigrate.ErrNoChange) {
			log.Fatalf("up: %v", err)
		}
		reportVersion(m)

	case "down":
		n := -parseStep(flag.Arg(1))
		if err := m.Steps(n); err != nil && !errors.Is(err, gomigrate.ErrNoChange) {
			log.Fatalf("down: %v", err)
		}
		reportVersion(m)

	case "version":
		v, dirty, err := m.Version()
		if errors.Is(err, gomigrate.ErrNilVersion) {
			fmt.Println("no schema_migrations row (database is empty / not yet migrated)")
			os.Exit(0)
		}
		if err != nil {
			log.Fatalf("version: %v", err)
		}
		fmt.Printf("version=%d dirty=%v\n", v, dirty)

	case "force":
		v, err := strconv.Atoi(flag.Arg(1))
		if err != nil {
			log.Fatalf("force: bad version %q: %v", flag.Arg(1), err)
		}
		if err := m.Force(v); err != nil {
			log.Fatalf("force %d: %v", v, err)
		}
		fmt.Printf("forced to version %d\n", v)

	case "drop":
		log.Println("WARNING: dropping all database objects")
		if err := m.Drop(); err != nil {
			log.Fatalf("drop: %v", err)
		}
		fmt.Println("dropped")

	default:
		fmt.Fprintf(os.Stderr, "unknown command %q\n\n", cmd)
		flag.Usage()
		os.Exit(2)
	}
}

func parseStep(s string) int {
	if s == "" {
		return 1
	}
	n, err := strconv.Atoi(s)
	if err != nil || n <= 0 {
		log.Fatalf("step must be a positive integer, got %q", s)
	}
	return n
}

func reportVersion(m *gomigrate.Migrate) {
	v, dirty, err := m.Version()
	if errors.Is(err, gomigrate.ErrNilVersion) {
		fmt.Println("no schema_migrations row (empty database)")
		return
	}
	if err != nil {
		log.Fatalf("version: %v", err)
	}
	fmt.Printf("schema_migrations: version=%d dirty=%v\n", v, dirty)
}