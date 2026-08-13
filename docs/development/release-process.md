# Release process

1. Set one version across the library, CLI, TUI, store, tests, and xtask path
   dependencies; update `CHANGELOG.md`.
2. Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets`, and
   `cargo test --workspace`.
3. Check optional features individually (`jsonata`, `pgp`, and `tesseract` on a
   host with its native libraries).
4. Regenerate operation pages with `cargo run -p xtask -- docs`, regenerate the
   combined reference example, and confirm both `--check` modes pass.
5. Build Read the Docs exactly as CI does: `mkdocs build --strict`.
6. Inspect `cargo package --list` and run `cargo package` for publishable crates.
7. Tag `vVERSION` only after CI is green; publish dependency crates before the
   CLI/TUI crates that refer to them.

Never release an operation that silently emits a placeholder as a real result.
Either implement and test it or mark it broken in runtime metadata.
