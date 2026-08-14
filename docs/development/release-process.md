# Release process

1. Set one version across the library, CLI, TUI, store, tests, and xtask path
   dependencies; update `CHANGELOG.md`.
2. Run `./scripts/check-platforms.sh`, or run the Linux, Windows cross-build,
   and native-host scripts individually.
   The same gates run automatically through
   `.github/workflows/platform-checks.yml` for pull requests and pushes to
   `main`.
3. Review the terminal output and collect benchmark JSON from
   `target/benchmarks/` when performance comparison is required.
4. Review `cargo package --list` and package every independently publishable
   crate before publishing anything.
5. Tag and publish manually only after the Docker gate is green. Publish
   dependency crates before CLI/TUI crates that refer to them.

Release archives must contain the platform binary, `rxchef.1`, Bash/Zsh/Fish/
PowerShell completions, release notes, and SHA-256 checksums. Smoke-test each
archive with `--version`, `--help`, `operations --json`, `run "To Base64"`, and
the exact binary roundtrip fixture before publishing it. Only advertise targets
that the release pipeline built and exercised. Linux uses the complete container
gate, Windows has a reproducible cross-build container plus the native gate, and
macOS uses the native gate because its SDK and linker are supplied by Xcode.

The pipeline writes build output under `target/`, Cargo caches, and temporary
directories. It does not create versioned release reports or benchmark snapshots
inside the source tree.

Never release an operation that silently emits a placeholder as a real result.
Either implement and test it or mark it broken in runtime metadata.
