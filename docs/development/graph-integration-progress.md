# Graph integration continuation — 2026-10-14

**Historical checkpoints only.** The dates, counts and remaining-work tables
below are preserved as development history and are not current release gates.
Use the [consolidated release report](0.0.1-release.md)
for the current implementation, fresh evidence and release blockers.

Updated 2026-10-15: stored graph/project repositories, portable session contracts,
browser project library/import/export and metadata history are implemented.
The first complete pre-persistence workspace regression passed **2,478 tests**;
new persistence/event changes require the subsequent full regression gate.
Current frontend unit/lint checks pass (56 tests); real browser E2E passes all
12 scenarios, including live node status and project reload/rebinding.
Native worker tests pass all five scenarios including actual memory termination
and node-start/interruption metadata. Windows worker/resource code cross-compiles
with the Windows GNU target; native Windows runtime remains NOT TESTED.

Protocol 2 is intentional: authoritative graph port contracts and live metadata
must not be mistaken for protocol-1 server compatibility. Native dialog and
installer evidence still remains open. The tables below retain the earlier
checkpoint; these additions supersede the earlier missing-persistence/live-event
statements. A consolidated release report follows the final gates.

Baseline: clean `master` at `90ca88064fdbd60a82a4a77ac040c7823eb742d8`.
This is an implementation checkpoint, not release approval. No publication,
push, upload, version change or tag was performed.

## Implemented changes

- Rust descriptors now own argument port IDs/types and graph-operation eligibility.
  All 478 descriptors are covered by parity tests. Byte adapters preserve exact
  bytes; textual adapters use the existing argument parser. NFC identifiers,
  orphan positions, labels, port counts and selected outputs are checked.
- Web graph runs freeze definitions and inputs with run/graph/input/connection
  revisions. Changes invalidate outputs; older runs are cancelled or discarded.
  Shared recipe/graph job handling also cancels submissions that finish after
  their UI owner closes or changes connection.
- Server and native clients share the process supervisor and bounded job manager
  in `rxchef-client`. Desktop and TUI invoke their own binary's internal worker
  entry point before initializing UI/terminal state. Cancellation kills and reaps
  workers; parent lifeline closure also terminates children. Queue, deadline and
  retained-output bounds remain enforced.
- Graph editor components now separate canvas, registry search, inspector,
  argument controls, embedded recipe steps, execution and outputs. All six
  combines, byte arguments and recipe nodes are editable. DAG layout accounts
  for measured sizes. Selection copy/paste and keyboard commands are available.
- Desktop graph save/load/output export use native dialogs. File replacement is
  atomic. The native job-routing registry has 64 admission permits and keeps job
  calls associated with the client that submitted them.
- TUI has separate client/session, background task and graph-screen modules.
  Graph file/directory selection, bindings, asynchronous jobs, cancellation,
  node metadata, named output selection, bounded previews and atomic export work.
  Recipes also use supervised jobs in the running application. Remote configuration
  is not labelled as a confirmed connection before any request succeeds.
- CLI doctor returns failure when a requested remote check fails and distinguishes
  PASS, WARN, FAIL and NOT TESTED. It does not claim untested installations work.
- Browser E2E builds the current frontend and web-enabled server before testing,
  avoiding stale or accidentally headless binaries.

## Local evidence

Platform: macOS Apple Silicon. These are local results, not claims about CI.

| Check | Result | Evidence |
|---|---|---|
| Core graph tests | PASS | 19 tests, including all-byte adapters and descriptor parity |
| Frontend typecheck/lint | PASS | Current graph/recipe changes |
| Frontend unit tests | PASS | 54 tests; one concurrent-build timeout was followed by a successful isolated run |
| Frontend contracts/build | PASS | Rust generation check, TypeScript and Vite |
| Real browser E2E | PASS | 10 tests: three inputs, fan-in/out, multiple exact downloads, stale revisions, cancellation, recipe/argument nodes, AES key/IV byte inputs, invalid port type, recipe/session workflows |
| Native backend tests | PASS | 3 tests: recipe/graph bytes and atomic replacement |
| Native app bundle | PASS | Debug macOS `.app` built; UI-test feature enabled |
| Native bundle functional smoke | PASS | Registry/UI load, offline recipe Bake, all 256 bytes via IPC, byte-adapter/embedded-recipe graph, real CPU-bound job cancellation; test app exits |
| Native dialog interaction / installation | NOT TESTED | Dispatch and export-file unit tests do not substitute for native dialogs or installed-package tests |
| TUI unit tests | PASS | 8 tests including real Core execution, exact export, revision invalidation, input errors and small terminal rendering |
| Real local/remote TUI PTY | PASS | File/directory selection, bindings, exact export, save/reload, cancellation, resize, Ctrl+C, restored flags, restart and server disconnect during an active job |
| CLI diagnostics regression | PASS | Unreachable requested server yields nonzero exit; unchecked remote/native checks remain explicit |
| Full workspace / release gates | NOT TESTED | Must be rerun after remaining integration work |
| Windows/Linux installer and native smoke | NOT TESTED | No native-runner evidence in this local session |

Reproducible commands include:

```sh
cargo test --locked -p rxchef graph:: --lib
npm --prefix apps/web run typecheck
npm --prefix apps/web run lint
npm --prefix apps/web run test:unit
npm --prefix apps/web run test:e2e
cargo test --locked -p rxchef-gui --bin rxchef-gui
cargo test --locked -p rxchef-tui
cargo build --locked -p rxchef-tui
python3 scripts/tui-graph-smoke.py target/debug/rxchef_tui --server-binary target/debug/rxchef-server
cargo test --locked -p rxchef-cli --test cli doctor_reports
```

Native app evidence uses `npm --prefix apps/web run desktop:build -- --debug
--bundles app --features ui-test`, followed by a background launch of the actual
bundle with `--ui-test-output=PATH`. The generated JSON asserts bytes and real
worker cancellation. It is a development test artifact, not a release installer.

## Remaining acceptance work

The binding assignment's milestone numbers supersede the earlier report's numbering.

| Milestone | Status | Outstanding acceptance |
|---|---|---|
| M1 correctness | IMPLEMENTED / TESTED | Final broad regression checks still required |
| M2 execution lifecycle | PARTIAL | True streaming per-node events; platform memory containment (macOS CPU/deadline bounds, Windows deadline/kill; Linux address-space limit) |
| M3 editor | PARTIAL | Remaining trace UX, project/session persistence and broader keyboard/theme/session E2E coverage |
| M4 desktop | PARTIAL | Native dialogs, close-during-work, remote transition and installation/reinstallation evidence |
| M5 terminals | PARTIAL | CLI signal/compatibility and the complete terminal acceptance matrix |
| M6 persistence | MISSING | Shared stored graphs, multi-definition projects and bounded graph history integration |
| M7 release pipeline | NOT TESTED | RC version discovery, native installers, package checks, platform runners |
| M8 release candidate | NOT TESTED | Final security/performance review and complete release-readiness evidence |

Current release classification remains **Internal Alpha**. Missing persistence,
live events, platform containment and installer evidence prohibit an RC claim.
