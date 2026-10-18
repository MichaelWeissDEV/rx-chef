# CI and release-readiness report

Assessment date: 2026-10-18.  This report covers local `master` at
`4379d809`; that commit is local-only and has not been pushed.

## Failed-job root causes and correction

| Job | Root cause | Correction and evidence |
| --- | --- | --- |
| Windows native publishing tests | Git Bash/MSYS2 rewrote the POSIX fixture `PATH` inherited from Python. Git's real Cargo and curl could win command lookup, producing the fixture assertion failure and 30-second network timeouts. | `scripts/test-publish-crate.py` now starts a profile-free Bash, assigns the fixture path inside that shell, validates `cargo`, `curl`, and `sleep` there, and executes the helper in that same environment. The curl fixture invokes its companion Python program through an explicit interpreter. The 12 local publishing regressions pass. Native Windows validation on this commit is **NOT TESTED**. |
| Linux release container | The known CI job reaches the RustSec gate after the other repaired server and packaging gates. It reports vulnerable direct/transitive dependencies (RSA, Wasmtime and ZIP paths); the configured policy keeps this gate blocking. | No ignore rule, disabled feature or failure masking was added. The dependency analysis and mitigation status are recorded in `dependencies.md`. Local container execution is **NOT TESTED** because the Docker daemon is unavailable. |

## Local validation

Passed on the current commit:

| Check | Result |
| --- | --- |
| `python3 scripts/test-publish-crate.py -v` | PASS, 12 tests |
| `python3 scripts/test-release-version.py` | PASS, 13 tests |
| `python3 scripts/test-distributions.py` | PASS, 12 tests |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --locked --workspace --all-targets` | PASS |
| `cargo check --locked --workspace --all-targets --all-features` | PASS |
| `cargo test --locked --workspace` | PASS |
| `cargo test --locked --workspace --all-features` | PASS |
| `cargo clippy --locked --workspace --all-targets -- -D clippy::correctness -D clippy::suspicious` | PASS; pre-existing non-fatal warnings remain |
| `npm --prefix apps/web ci` | PASS |
| `npm --prefix apps/web run build` | PASS |
| `npm --prefix apps/web test` | PASS, 107 tests |
| `actionlint` | PASS |

The host-mode release gate also passed version, distribution and publishing
regressions, contracts, TypeScript, frontend lint/build/tests, registry and
operation audit, fuzz-target compilation, documentation, all release binary
builds, CLI/server smokes, four archive builds, inventory, and cargo-dist plan
and host archive generation.

## Host gate limitations and failures

`scripts/release-check-linux.sh` could not use its authoritative Linux
container because `docker info` cannot contact a daemon. The fallback
`--host` run failed its summary. Its RustSec failure is release-blocking. The
frontend dependency audit could not resolve the npm advisory endpoint because
this environment has no DNS/network access. Browser E2E, HTTP protocol,
remote-client, web-server, graph-smoke and archive server-smoke substeps all
failed when the sandbox forbade a local `127.0.0.1` listener with
`Operation not permitted`; they are environment-limited, not code regressions.

The macOS DMG bundler built the app then returned failure from Tauri's generated
`bundle_dmg.sh`. Its detailed invocation is not retained by Tauri, so this
requires validation on an unrestricted native macOS release runner before a
release claim. Linux-only FFI C linking, package/install verification and
benchmarks were explicitly skipped in host mode. Native Windows and all GitHub
checks on `4379d809` are **NOT TESTED** because no remote changes were allowed.

## Changed files

- `scripts/test-publish-crate.py`: Windows fixture isolation and early command
  resolution validation.
- `verification/first-public-release/ci-windows.md`: Windows root cause,
  local validation and native-run limitation.
- `verification/first-public-release/packaging.md`: current publishing-test
  count and coverage.
- This report.

## Release readiness

**NOT READY FOR RELEASE.** The mandatory RustSec dependency audit remains red.
Before release, run the unmodified security gate to green, execute native
Windows publishing tests and the complete Linux release container on this exact
commit, then review the macOS DMG result in an unrestricted native runner.
