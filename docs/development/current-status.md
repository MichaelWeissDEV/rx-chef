# Current status for v0.0.1

## Workspace Configuration
- **Version:** 0.0.1
- **Workspace Crates:** `rxchef` (lib), `rxchef_cli`, `rxchef_tui`, `rxchef_store`, `cyberchef-rust-tests`
- **Binaries:** `rxchef` (from `rxchef_cli`), though `Cargo.toml` specifies a default-run of `rxchef`.

## Features
Defined features in `rxchef`:
- `pgp` (depends on `sequoia-openpgp`)
- `jsonata` (depends on `jaq-core`, `jaq-std`)
- `tesseract` (depends on `tesseract-rs`, `leptonica-sys`)

## Build State
- `cargo check --workspace --all-targets --all-features`: passes.
- `cargo clippy --workspace --all-targets -- -D clippy::correctness -D clippy::suspicious`: is the CI correctness gate. Style lints remain advisory for the initial release.

## Tests
- `cargo test --workspace`: passes with 1,829 passing tests and 9 environment/fixture-dependent ignored tests.
- `cargo test --workspace --all-features`: passes with 1,830 passing tests and 9 ignored tests (including the all-operations availability invariant).
- All-feature registry conformance asserts that none of the 478 operations is marked broken.

## Known limitations
- Entries marked `broken` in a minimal build require the optional feature named in the feature matrix; they refuse to return placeholder results.
- OCR requires the optional `tesseract` feature and system Tesseract/Leptonica libraries.
- PGP is implemented and tested behind `--features pgp`.

## Environment
- OS: MacOS
- Rust version: rustc 1.96.1
- Cargo version: cargo 1.96.1
