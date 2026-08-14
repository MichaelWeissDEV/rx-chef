# Release process

1. Set one version across the library, CLI, TUI, store, tests, and xtask path
   dependencies; update `CHANGELOG.md`.
2. Run the complete Linux x86_64 release gate: `./scripts/release-check-linux.sh`.
3. Inspect the generated Linux release report and benchmark environment.
4. Review `cargo package --list` and package every independently publishable
   crate before publishing anything.
5. Tag and publish manually only after the Docker gate is green. Publish
   dependency crates before CLI/TUI crates that refer to them.

Release archives must contain the platform binary, `rxchef.1`, Bash/Zsh/Fish/
PowerShell completions, release notes, and SHA-256 checksums. Smoke-test each
archive with `--version`, `--help`, `operations --json`, `run "To Base64"`, and
the exact binary roundtrip fixture before publishing it. Only advertise targets
that the local release pipeline actually built and tested. Linux x86_64 is the
intended target, but remains pending until the current tree completes the gate;
macOS and Windows are not verified.

Never release an operation that silently emits a placeholder as a real result.
Either implement and test it or mark it broken in runtime metadata.
