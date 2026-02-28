# AGENTS.md

## Cursor Cloud specific instructions

This is a Rust CLI tool (`lictl`) for scripting Lichess.org API operations. It is a single-crate Cargo workspace at `crates/lictl`.

### System dependencies

The build requires `libdbus-1-dev`, `libssl-dev`, and `pkg-config` on Linux (for `keyring` and `openssl-sys` crates). These are pre-installed in the snapshot.

### Common commands

See `README.md` for full usage. Key dev commands:

- **Build:** `cargo build`
- **Test:** `cargo test` (unit tests use `mockito` for HTTP mocking; no external services needed)
- **Lint:** `cargo fmt --check` and `cargo clippy -- -D warnings`
- **Format:** `cargo fmt --all`
- **Run:** `cargo run -- <subcommand>` (e.g., `cargo run -- --help`)

### Gotchas

- **Rust edition 2024** requires Rust 1.85+. The update script runs `rustup update stable` to ensure this.
- The `keyring` crate uses `linux-native` feature on Linux, which depends on `libdbus`. Without `libdbus-1-dev`, the build will fail.
- The `openssl-sys` crate needs `libssl-dev`. Without it, the build will fail with an `OPENSSL_DIR` error.
- Most CLI commands require Lichess authentication (`lictl login`), but `games export` and `--help` work without auth.
- The `req get` command prepends `https://lichess.org/api` to the path, so use paths like `/account` not `/api/account`.
