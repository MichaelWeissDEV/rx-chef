# RC baseline — 2026-10-15

Initial commit: `de287614b1082e85e9619de6cd9178353a992f4f`. Branch: `master`.
Initial Git status: clean. Only existing tag: `v0.1.0`. No tags created.
Platform: macOS ARM64. Rust/Cargo 1.97.1; Node v26.7.0; npm 11.19.0.
Frontend and Tauri application version: 0.2.0-rc.1; Tauri crate/CLI pinned by respective lockfiles.
Cargo metadata --locked --format-version 1 --no-deps: PASS.

| Package | Version |
|---|---|
| rxchef | 0.2.0-rc.1 |
| rxchef-cli | 0.2.0-rc.1 |
| rxchef-client | 0.2.0-rc.1 |
| rxchef-store | 0.2.0-rc.1 |
| rxchef-tui | 0.2.0-rc.1 |
| rxchef-server | 0.2.0-rc.1 |
| rxchef-gui | 0.2.0-rc.1 |
| cyberchef-rust-tests | 0.2.0-rc.1 |
| xtask | 0.2.0-rc.1 |

Existing suites: Rust workspace/core operations/graphs; client contracts and worker supervisor;
server REST/RPC/WebSocket/worker tests; store repositories; CLI executable tests; TUI
unit plus POSIX PTY smoke; Tauri backend; frontend Vitest and Playwright.
Existing workflows: platform-checks, release, validate-tag, release-gates,
release-dry-run, build-deb/rpm/aur, publish-crates.

Known initial defects: release tag truncates prerelease before Cargo/changelog comparison;
graph edits invalidate results without immediate cancellation. cargo-dist absent initially.
First complete baseline attempted before runtime edits: failed at local TCP bind in sandbox
(2203 passed, 1 failed; remaining suites not reached). Unsandboxed rerun is separately recorded.

All measured command records and full stdout/stderr: `target/rc-evidence/*.json` and `*.log`.
These local build artifacts are not committed. Each record includes command, features,
start time, duration, exit code, platform, toolchain, commit, dirty state and available test counts.
A dirty-state run is evidence for the working tree, not certification of the original HEAD.
