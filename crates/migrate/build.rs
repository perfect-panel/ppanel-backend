// rust2go build script.
//
// Wires up the Go source tree (./go/r2g) into the Rust build:
//   - Embeds the Go source path so the Builder can compile it via CGO.
//   - Wires `src/idl.rs` (Rust IDL) → `./go/r2g/gen.go`
//     (auto-generated Go glue).
//
// On every `cargo build` the Builder regenerates gen.go if the IDL
// changed, then compiles the Go code into a static library that the
// Rust crate is linked against.
fn main() {
    rust2go::Builder::new()
        .with_go_src("./go/r2g")
        // gen.go is written relative to the build script's CWD (the
        // crate root), so we spell out the path under go/r2g/ where
        // the Go package lives.
        .with_regen("src/idl.rs", "go/r2g/gen.go")
        .build();
}
