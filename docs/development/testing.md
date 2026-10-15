# Testing handbook

Run tests from the repository root. Use synthetic non-secret inputs and an
isolated `RXCHEF_HOME` for commands that mutate state. `--locked` prevents an
accidental dependency update; a lockfile mismatch is a failure to investigate.

## Choose a check by the changed contract

| Change | Focused command | What it establishes |
|---|---|---|
| Graph semantics/budgets | `cargo test --locked -p rxchef --lib graph::tests` | Exact bytes, typed ports, branch isolation, structural/output limits and cancellation boundaries |
| Portable sessions/repositories | `cargo test --locked -p rxchef-store --test workbench_acceptance` | Public API validation, persistence, quotas, process locks and failed-write preservation |
| Client jobs | `cargo test --locked -p rxchef-client jobs::tests` | Admission, retention expiry and cancellation/termination distinction |
| Worker protocol | `cargo test --locked -p rxchef-server --test native_worker` | Actual subprocess IPC, timeout/cancellation and parent lifeline |
| HTTP/auth/remote client | `cargo test --locked -p rxchef-client --features remote` and `cargo test --locked -p rxchef-server --features web-ui` | Transport and policy behavior against real server paths |
| Web state/transport | `npm --prefix apps/web run test:unit` | Revision ownership, cancellation races, strict byte parsing and graph editor state |
| Web user flows | `npm --prefix apps/web run test:e2e` | Rebuilt assets and real web-enabled Rust server in Chromium |
| Release version/tag rules | `python3 scripts/test-release-version.py` | SemVer, exact changelog/version matching, prerelease publishing guards |
| Documentation examples | `python3 scripts/test-workbench-docs.py --cli target/release/rxchef` | Actual documented project, recipe and binary graph commands |

The command named `test:e2e` rebuilds assets and the server/CLI before starting
Playwright. Running Playwright alone after editing Rust can accidentally test an
older executable. Unit tests use controlled transports to force races; E2E and
native worker tests provide separate real-process evidence.

Playwright retains failure screenshots and `trace.zip` under the ignored
`apps/web/test-results/` directory. Open a retained trace from `apps/web` with
`npm exec -- playwright show-trace test-results/TEST/trace.zip`. Test attachments
use Playwright's portable output paths, including the successful graph screenshot
and performance JSON. Use synthetic fixtures because traces include page and
request details.

## Frontend and contract checks

```sh
npm --prefix apps/web ci
npm --prefix apps/web run contracts:check
npm --prefix apps/web run typecheck
npm --prefix apps/web run lint
npm --prefix apps/web run test:unit
npm --prefix apps/web run test:e2e
```

Generated wire contracts originate in Rust. Change the authoritative schema,
regenerate with `contracts:generate`, review the diff and run the check again.
Do not weaken a runtime validator or hand-edit generated types to make a test
pass. Error-path tests should also assert that the old valid state survives.

Browser coverage includes recipe roundtrips, binary graph output, separate AES
bindings, project export/reload, multi-tab transactions, denied storage, corrupt
records and injected quota/commit-abort failures. Injected failure proves that
path's handling; it does not prove real disk exhaustion. Performance tests record
10/50/100-node measurements as attachments; those observations are not universal
latency or memory guarantees. Native webview dialogs and OS clipboard behavior
need their own acceptance evidence.

## Workspace, formatting and operation consistency

```sh
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo test --locked --workspace
cargo check --locked --workspace --all-targets --all-features
cargo test --locked --workspace --all-features
cargo clippy --locked --workspace --all-targets --all-features -- -D clippy::correctness -D clippy::suspicious
cargo run --locked -p xtask -- check-registry
cargo run --locked -p xtask -- audit-operations
cargo run --locked -p xtask -- verify-version
cargo run --locked -p xtask -- docs --check
cargo run --locked --example generate_operation_docs -- --check
mkdocs build --strict
```

A default-feature pass does not exercise all optional native integrations.
Missing native dependencies must be recorded rather than reclassified as a
pass. Rustdoc tests are part of the workspace command. Avoid competing Cargo
builds in the same target directory during a broad verification run. If stale
metadata errors occur, retain the failing log, identify affected build artifacts,
clean only those packages and rerun the complete affected check. Never delete
user data to repair a build cache.

When adding an operation, retain registry/test/doc parity. When changing a byte
path, include NUL, invalid UTF-8, all 256 byte values, empty data, boundary lengths
and independent expected output. A text preview assertion alone cannot prove
binary correctness. For limits, test exact-boundary success and one-over failure.

## Help, manuals and container deployment

`python3 scripts/test-command-help.py` exercises 19 CLI help paths, server/TUI
flags, all five shell completions and generated manpage output, without leaking
a session token. Both release and native CI gates invoke it after building the
shipped binaries.

`python3 scripts/test-container.py --image rxchef-server:local --variant web`
checks the actual non-root/read-only image, worker byte output and shutdown.
The separate container CI matrix builds web/headless images on Linux AMD64 and
ARM64, runs isolated and host-network tests and validates Compose. Local host
networking requires the platform prerequisites in the [Docker guide](../server/docker.md).

## Process, terminal and installed archives

```sh
cargo build --locked --release -p rxchef-cli -p rxchef-tui -p rxchef-server
python3 docker/server-smoke.py target/release/rxchef
python3 scripts/tui-graph-smoke.py target/release/rxchef_tui --server-binary target/release/rxchef-server
bash scripts/build-distributions.sh cli
bash scripts/build-distributions.sh tui
bash scripts/build-distributions.sh headless
bash scripts/build-distributions.sh web
python3 scripts/smoke-distributions.py --directory target/distributions
```

Use a fresh `RXCHEF_DISTRIBUTION_DIR` when rehearsing again: the packager refuses
to overwrite existing archives. Archive smoke verifies checksums/license,
extracts into temporary installations, executes from an unrelated current
directory, removes/reinstalls and repeats. It does not certify native GUI
installers, signing, Gatekeeper or Windows ACLs.

The POSIX PTY script drives the release TUI in local/remote mode, checks exact
files, cancellation, resize/disconnect and terminal flag restoration. Native
Windows console acceptance is separate. Worker tests use actual child processes
and length-framed IPC, including malformed/truncated frames and parent EOF;
platform-specific memory assertions must run on the corresponding native OS.

## Fuzzing and release rehearsal

```sh
cargo check --locked --manifest-path fuzz/Cargo.toml --bins
PATH="/path/to/cargo-dist-0.32.0/bin:$PATH" bash scripts/release-gates.sh host
```

Compilation of fuzz targets is not a fuzz campaign. See [fuzzing](fuzzing.md)
for instrumentation, corpora and bounded runs. The host release gate aggregates
all failures and explicitly lists SKIPPED and NOT TESTED checks. Its PASS applies
to checks actually executed on that host. Run the native matrix and the
[acceptance scenarios](acceptance-matrix.md) before approving an RC.

## Preserve evidence

Wrap a command to retain stdout/stderr and a machine-readable record:

```sh
python3 scripts/record-release-check.py --label graph-regression --features default -- cargo test --locked -p rxchef --lib graph::tests
```

Records under `target/rc-evidence/` include command, OS/architecture, toolchains,
HEAD, dirty status, features, UTC start, duration, exit code and parsed counts
where available. A missing count is unknown, not zero. Keep failed attempts and
successful reruns separately. If files change during a long run, describe exactly
which source state was covered; do not label a dirty-tree run as clean-commit
certification. The [current release report](0.0.1-release.md) links the
checked-in evidence summary and unresolved platform/GUI work.
