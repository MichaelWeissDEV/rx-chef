# Release process

1. Set one version across the library, CLI, TUI, store, tests, and xtask path
   dependencies; update `CHANGELOG.md`.
2. Run `cargo fmt --all --check`, the configured strict Clippy gate, and
   `cargo test --workspace`.
3. Check optional features individually (`jsonata`, `pgp`, and `tesseract` on a
   host with its native libraries).
4. Run `cargo xtask check-registry`, `cargo xtask audit-operations`, regenerate
   operation pages with `cargo xtask docs`, regenerate the combined reference,
   and confirm both docs `--check` modes pass.
5. Build Read the Docs exactly as CI does: `mkdocs build --strict`.
6. Inspect `cargo package --list` and run `cargo package` for publishable crates.
7. Tag `vVERSION` only after CI is green; publish dependency crates before the
   CLI/TUI crates that refer to them.

Release archives must contain the platform binary, `rxchef.1`, Bash/Zsh/Fish/
PowerShell completions, release notes, and SHA-256 checksums. Smoke-test each
archive with `--version`, `--help`, `operations --json`, `run "To Base64"`, and
the exact binary roundtrip fixture before publishing it. Only advertise targets
that the release workflow actually built and tested.

Never release an operation that silently emits a placeholder as a real result.
Either implement and test it or mark it broken in runtime metadata.
