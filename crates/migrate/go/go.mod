module github.com/perfect-panel/ppanel-backend/crates/migrate/go

go 1.25.0

require (
	github.com/golang-migrate/migrate/v4 v4.19.1
	// rust2go has no tagged releases; pin to the latest commit on master.
	// Bump by re-running `go get github.com/ihciah/rust2go@master`.
	github.com/ihciah/rust2go v0.0.0-20260706074211-1c851bd436aa
)

require (
	filippo.io/edwards25519 v1.1.0 // indirect
	github.com/go-sql-driver/mysql v1.8.1 // indirect
	github.com/lib/pq v1.10.9 // indirect
)
