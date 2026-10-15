# Development evidence and continuation boundary — 2026-10-14

**Historical checkpoint.** The entire document below, including its checklist
and next tasks, describes an earlier development state. The authoritative
current assessment is [0.2.0-rc.1 readiness](0.2.0-rc.1-release-readiness.md).

This records the previous checkpoint. For the subsequent binding assignment and
current implementation evidence, see [the continuation report](graph-integration-progress.md).

This is an implementation checkpoint, **not a completed first production release**.
The shared graph domain, local/remote API, CLI and initial visual editor are usable
and tested on macOS Apple Silicon. Remaining mandatory work is recorded below.
No push, tag, upload, crate publication or external release was performed.

## Baseline and preparation

- Initial HEAD: `558808cbab818f70ed70bee62314761205fcdf9f`.
- Initial branch: `master`; existing version and local tag: `0.1.0` / `v0.1.0`.
- The checkout already contained extensive changes to the web workbench, desktop,
  client/server, TUI, lockfiles and release scripts. They were preserved.
- Initial status and a binary diff backup were captured at
  `/private/tmp/rxchef-initial-status.txt` and `/private/tmp/rxchef-existing-work.patch`.
- Reviewed Cargo workspace/dependencies, frontend package graph, core runtime and
  Recipe Engine, client policy/wire contracts, supervised server workers/jobs,
  CLI dispatch, TUI background execution, Store recipe/project/history APIs and
  cargo-dist/release/native-platform gates.
- Reviewed local ts-chef graph model, runner, webview, documentation and test cases.
  Retained stable ordering, upstream sharing, disconnected-node treatment,
  branch failure isolation and strict conversion principles; extended fan-in.
- The user subsequently supplied F-01 through F-07. Their current source and
  existing runtime evidence are reconciled in the review table below.
- No new release number was chosen. Published version history is **NOT TESTED**;
  check it before selecting an RC version.

## Reconciliation of the seven supplied review findings

The supplied prior review established source fixes but explicitly lacked complete
build/runtime evidence. The following statuses use the current source and the
successful checks recorded during this development session. No fresh full security
scan or cross-platform certification is implied.

| Finding | Status | Current evidence |
|---|---|---|
| F-01: node_modules tracked in Git | PASS | `git ls-files '**/node_modules/**'` returns no entries; `git check-ignore` confirms installed frontend dependencies are ignored |
| F-02: token persisted in localStorage | PASS | Token lives in React memory; legacy local/session storage keys are removed; real browser E2E checks both storage locations and authentication after reload |
| F-03: static assets blocked by auth | PASS | Web-enabled GET/HEAD assets are public while API access stays protected; real browser E2E loads the page/assets before entering a token and checks unauthenticated operations return 401 |
| F-04: Rust/TypeScript contracts inconsistent | PASS | Authoritative Rust schemas generate TS models and runtime validators; fixture/schema tests and `contracts:check` pass; this check now gates normal frontend builds and native-platform checks as well as existing release/workbench gates |
| F-05: unsafe HEX parsing / byte conversions | PASS | Strict HEX/Base64 and distinct strict UTF-8 versus display previews; malformed-input, all-byte, NUL/invalid-UTF-8 and large-buffer unit tests; exact browser import/download comparisons |
| F-06: WebSocket subscription race | PASS | Owned semaphore permit acquired before spawning and retained for subscription lifetime; real subscription-cap test submits 100 subscriptions and accepts exactly 2 at configured capacity 2; flood/disconnect cleanup tests also pass |
| F-07: assets depend on working directory | PASS | Build-time `include_bytes!` embedding; E2E server starts with `cwd: tmpdir()` and successfully serves the workbench and its assets |

F-06 PASS covers the supplied capacity race and exercised cleanup scenarios; it is
not proof of every possible scheduling interleaving. F-07 verifies development
binary execution from another directory, not installed-package correctness.

The prior review's remaining-work list is now partly superseded: recipe save/load,
undo/redo, frontend unit/E2E tooling, contract npm scripts, the Tauri project/native
Rust dispatch, and TUI background/remote code exist. Browser and backend evidence
is recorded below; native GUI, interactive TUI and installed-package smoke tests
remain unverified. Required graph-specific and release work remains open.

## Architecture and implemented semantics

Workspace: root `rxchef` library; `crates/{client,server,store,cli,tui,xtask}`;
`apps/web`; `apps/desktop/src-tauri`; and the independent `tests` crate.

`rxchef::graph` owns the versioned model, whole-definition validator, stable
lexical DAG plan, selected-output dependency closure and exact-byte executor.
Operations call the existing registry/runtime. Embedded recipes call the existing
Recipe Engine, including its control flow and step bounds. No transformation
algorithm is reimplemented by transports or UI code.

Data edges carry bytes, with explicit registry-typed argument adapters for
non-byte arguments. AES byte-valued argument bindings encode using the runtime's
typed prefixes. Immutable `Arc<[u8]>` values support fan-out. Each required node
executes once; intermediate values are released after their last consumer.
Failed nodes block downstream dependencies; unrelated outputs remain available.
Output errors carry no partial payload. Graph error metadata excludes argument
values and intermediate data.

Client contracts add `GraphRequest`, `GraphResult`, graph capability discovery,
local/remote methods and graph task/result variants. Generated Rust schemas,
TypeScript types, runtime validators and fixtures include the new contract.
Server REST, JSON-RPC and supervised jobs share the same worker path. One graph
runs in one worker process; cancellation and deadlines terminate that worker.
Existing auth, scope, Host/Origin and resource controls remain authoritative.

The web/desktop workbench adds a lazy-loaded React Flow canvas, inspector,
input/file binding, combine ports, connection controls, graph undo/redo,
arrangement, import/export, output tabs, previews and complete binary downloads.
Edits never trigger execution. Remote runs use the supervised jobs API. Output
payload/status/count consistency is checked before presentation. Browser Save
refuses sensitive literals and never includes input bytes or result artifacts.
Native dispatch exposes graph methods without requiring a server.

CLI adds `graph validate/run/inspect/list/import/export`. It reads explicitly
selected local files with bounds, transfers only bytes remotely, keeps diagnostics
on stderr, rejects ambiguous binary stdout and atomically writes named outputs.
The local packaging helper now uses the actual `rxchef_tui` binary filename.

## Milestone status at this boundary

| Milestone | Status | Concrete remaining work |
|---|---|---|
| M0 baseline/stabilization | FAIL | Build/test baseline and F-01–F-07 checks PASS; full platform/release gates not certified |
| M1 Rust graph domain | FAIL | Core semantics/tests PASS; parallel scheduling, richer tracing/metadata and remaining release cases incomplete |
| M2 client/server graphs | FAIL | REST/RPC/jobs/equality/cancellation PASS; live graph event contract incomplete |
| M3 visual editor | FAIL | Tested multi-input browser workflow PASS; advanced required interactions incomplete |
| M4 complete web workbench | FAIL | Full graph project persistence, trace UX and remaining daily-use features incomplete |
| M5 desktop | FAIL | Native backend dispatch PASS; installed GUI/offline/platform smoke absent |
| M6 terminal clients | FAIL | Initial CLI graph commands PASS; TUI graph workflows missing |
| M7 release candidate | NOT TESTED | Full gate, security/performance/installer/platform evidence absent |

These statuses measure the assignment's complete milestone definitions, not whether
individual delivered features work. No milestone is declared complete early.

## Frontend evidence

| Component | Implemented | Tested | Partial / missing |
|---|---|---|---|
| Core | Version 1 graphs, named inputs, typed argument ports, six combine kinds, fan-in/out, multiple outputs, recipes, validation, limits, cancellation checks | 14 graph unit tests plus existing engine/regression suite | Sequential scheduler; parallel branches, event streaming and future migrations remain |
| CLI | Six graph commands, named local inputs/outputs, optional remote client, JSON and raw binary modes | Real executable tests for validation, named files, exact bytes and ambiguous stdout; remote engine API tests | Store-integrated graph library and richer diagnostics remain; CLI remote end-to-end invocation not separately tested |
| TUI | Existing local/remote background recipe execution retained | Existing TUI unit tests and workspace build | Graph navigation/binding/execution/export missing; interactive terminal smoke not performed |
| Server | Graph REST/RPC/jobs using supervised workers | Real server tests: equality, auth/scopes, limits, cancellation, deadlines; browser runs against embedded web server | Streaming graph node events missing; platform memory restrictions remain |
| Browser | Initial real canvas and inspector, multiple input/combine/output nodes, file bytes, named outputs, save/load, undo/redo, remote Run/Cancel | Five Chromium E2E scenarios, 42 frontend unit tests, lint/typecheck/build/contracts | Advanced interactions, project persistence, accessible graph command palette, typed adapter creation, recipe editing, conversion controls and full trace payload inspection remain |
| Desktop | Shared frontend and native graph dispatch, local/remote Rust clients | Two native dispatch unit tests; workspace compile | Native window launch, installed/offline app smoke, file-dialog graph workflow and installers NOT TESTED; local hard cancellation missing |
| Store | Existing recipe/project/history behavior retained | Existing workspace tests | Graph-specific project/session/history persistence missing |

## Real graph examples

1. `examples/graphs/linear.rxgraph.json`: one input → Base64 → named output.
2. `three-inputs.rxgraph.json`: three binary inputs, shared encode/decode fan-out,
   concatenation and three named outputs. Core, client, real server REST/RPC/jobs,
   CLI files and browser downloads verify byte equality.
3. The combine tests verify comparison, strict equal-length XOR, ordered
   concatenation, UTF-8 joins and unequal-length interleaving.
4. `aes-decrypt.rxgraph.json`: ciphertext, key and IV feed real registered AES.
   The unit test encrypts known binary data with the runtime and checks decryption
   byte-for-byte through separately bound ports.
5. Browser E2E builds a three-input graph entirely through controls, downloads all
   three outputs, saves/reloads the definition, rebinds data and rechecks output.
6. `branch-isolation.rxgraph.json` and unit tests preserve an unrelated output
   while an AES branch fails. Failure propagates only to its dependencies.
7. Server tests compare local, REST, RPC and graph-job outputs using exact bytes,
   including every byte `00..FF`, NULs and invalid UTF-8.

## Test commands and results

Platform: Darwin arm64; Rust `1.97.1`; Node `26.7.0`. Socket/browser tests ran with
loopback access because the filesystem/network sandbox initially denied binding.

| Command | Status | Evidence |
|---|---|---|
| `cargo fmt --all -- --check` | PASS | Formatting current |
| `cargo check --locked --workspace --all-targets` | PASS | Includes native desktop backend |
| `cargo test --locked --workspace` | PASS | 2,465 passing test cases including doctests in the successful full run |
| `cargo test --locked -p rxchef graph:: --lib` | PASS | 14 graph cases |
| `cargo test --locked -p rxchef-client --features remote` | PASS | Local contracts and loopback HTTP fixtures |
| `cargo test --locked -p rxchef-server` | PASS | 24 real server cases in full workspace run |
| `cargo test --locked -p rxchef-server -p rxchef-cli -p rxchef-client --features rxchef-client/remote` | PASS | Final self-contained packaged fixtures; real executable/transport tests |
| `cargo clippy --locked --workspace --all-targets -- -D clippy::correctness -D clippy::suspicious` | PASS | Other existing advisory warnings remain |
| `cargo run --locked -p xtask -- check-registry` | PASS | Registry current |
| `cargo run --locked -p xtask -- audit-operations` | PASS | 478 operations; audit still lists negative/independent-evidence gaps |
| `cargo run --locked -p xtask -- docs --check` | PASS | 478 generated operation documents current |
| `npm --prefix apps/web run typecheck` | PASS | Includes graph feature |
| `npm --prefix apps/web run contracts:check` | PASS | Rust schemas/types/runtime validators/fixtures current |
| `npm --prefix apps/web run lint` | PASS | Includes graph feature and tests |
| `npm --prefix apps/web run test:unit` | PASS | 42 tests |
| `npm --prefix apps/web run test:e2e` | PASS | Five real-server Chromium scenarios |
| `npm --prefix apps/web run build` | PASS | Graph code split into lazy chunk; main bundle still exceeds Vite warning threshold |
| `cargo build --locked -p rxchef-server --features web-ui` | PASS | Embedded web assets served during E2E |
| Full native/platform/release gate scripts, all-features builds/tests | NOT TESTED | Not certified by default-feature success |
| Linux x86_64/aarch64, macOS Intel, Windows x86_64 launch/install tests | NOT TESTED | No evidence produced on these platforms |
| Native desktop GUI/installer/offline smoke; TUI terminal interaction | NOT TESTED | Compile/dispatch unit tests do not substitute for launch evidence |
| Performance benchmark suite and final exhaustive security review | NOT TESTED | No production readiness claim |
| Package installation/uninstallation/reinstallation and release dry run | NOT TESTED | No release packages produced |

Earlier failures were retained in the session logs and resolved without skipping
tests: the first branch-isolation fixture used permissive From Hex and was
replaced with a genuinely failing AES operation; a desktop test referenced an
undeclared crate and was corrected; one busy frontend run exceeded its 5-second
large-buffer test timeout and passed in isolated runs; browser reload originally
rebound inputs before import validation completed and now waits for the visible
completion state. Initial server tests failed on denied socket binding and passed
with loopback access. Concurrent Cargo feature builds replaced the core crate's
shared `librxchef.rlib` during doctests, causing E0460/E0463; serial full/targeted
runs pass. Run Cargo test/build/schema commands serially with this target directory.

## Artifacts actually produced

- Development CLI `target/debug/rxchef`.
- TUI and native-backend test executables under `target/debug/deps/`.
- Development `target/debug/rxchef-server`, including a web-enabled build used
  by browser tests; later default builds may replace the same executable path.
- `apps/web/dist/` production web assets and staged server web assets.
- Generated contracts and four example graph definitions.
- No release archives, installers, signed applications or published Rust crates.
  Target package names remain `rxchef`, `rxchef-store`, `rxchef-client`,
  `rxchef-cli` (`rxchef` executable), `rxchef-tui` (`rxchef_tui`),
  `rxchef-server`, and `rxchef-gui`.

## Working development commands

```sh
# Web-enabled local server; copy the generated token from stderr into the UI.
npm --prefix apps/web run build
cargo run --locked -p rxchef-server --features web-ui -- --listen 127.0.0.1:8787
# Open http://127.0.0.1:8787 and choose Graph mode after connecting.

# Headless server, standalone native development app, TUI, basic transform.
cargo run --locked -p rxchef-server -- --listen 127.0.0.1:8787
npm --prefix apps/web run desktop:dev
cargo run --locked -p rxchef-tui
cargo run --locked -p rxchef-cli -- run to_base64 --input Hello

cargo run --locked -p rxchef-cli -- graph validate --graph examples/graphs/three-inputs.rxgraph.json
cargo run --locked -p rxchef-cli -- graph run \
  --graph examples/graphs/three-inputs.rxgraph.json \
  --input payload=payload.bin --input second=second.bin --input third=third.bin \
  --output combined=combined.bin --output original=original.bin --output encoded=encoded.bin

# Local release verification only; not run/certified in this checkpoint.
bash scripts/release-gates.sh host
```

See `docs/concepts/graphs.md` for port semantics, remote CLI selection and format
limitations. Desktop and TUI launch commands above are the configured development
entrypoints; successful interactive launches have not been certified here.

## Mandatory release checklist

| Requirement | Status | Reason / scope |
|---|---|---|
| Existing 478 operations under complete supported feature matrix | NOT TESTED | Default registry/regression PASS; all-features/platform combinations not run |
| Linear recipes execute correctly | PASS | Existing regression and browser tests |
| Graphs support multiple input nodes | PASS | Core, server, CLI and browser tests |
| Genuine multi-input operations | PASS | Combine nodes and AES named bindings |
| Branches and multiple named outputs | PASS | Exact output comparisons/downloads |
| Graph execution shared across all frontends | FAIL | TUI graph interface missing |
| Local/remote results byte-identical | PASS | Real server REST/RPC/jobs |
| Browser workbench fully operational | FAIL | Initial graph flow works; required advanced/editor/project features incomplete |
| Desktop has working native backend | PASS | Compile plus local graph/recipe dispatch unit tests |
| Desktop GUI works independently without server | NOT TESTED | Backend dispatch PASS; native installed GUI not launched |
| Headless server independently buildable | PASS | Default standalone target compiled and exercised |
| Installed server-with-Web-UI works | NOT TESTED | Development embedded web server PASS; installation not tested |
| CLI graph execution and named outputs | PASS | Real executable tests |
| TUI local/remote execution | NOT TESTED | Unit/build evidence exists; interactive end-to-end not verified |
| Graphs and recipes saved/loaded | PASS | Explicit file workflows; full project persistence remains incomplete |
| Arbitrary binary data survives import/run/export | PASS | Core/client/CLI/browser byte comparisons |
| Resource limits and cancellation | FAIL | Remote worker cancellation PASS; local hard interruption and parallel budgeting incomplete |
| Auth and side-effect restrictions | PASS | Existing and added server policy tests; not an exhaustive security certification |
| Relevant unit/integration tests | PASS | Commands above |
| Browser E2E | PASS | Five real-server scenarios, not every required future UI feature |
| Desktop smoke on all supported platforms | NOT TESTED | No GUI launch/platform certification |
| Packaging and installation | NOT TESTED | No installer evidence |
| Generated documents/contracts current | PASS | Registry/docs/contracts gates |
| Reproducible release dry run | NOT TESTED | Full release gate not run |
| No critical/high release blockers | FAIL | Mandatory product work and platform/release evidence remain |

## Exact next executable tasks

1. Finish graph-domain release requirements: bounded parallel branch scheduling
   with deterministic outcomes, remaining input-source/view metadata, per-operation
   linear recipe conversion, event streaming/trace previews with explicit
   sensitive-data retention, a clear migration interface, and remaining validator/
   output-budget/select/disabled-node/property tests. Keep shared Rust authority.
2. Add versioned Store graph/project persistence and bounded session history.
   Bind payloads separately; do not persist secrets automatically.
3. Finish editor controls: graph-aware searchable operations/command palette,
   typed argument adapters and embedded recipe editing, list↔graph conversion,
   drag/drop, duplicate/copy/paste/reconnect/grouping, undo coalescing,
   per-node live tracing, and complete accessibility/narrow-layout tests.
4. Add TUI graph file selection, named input binding, jobs/status/output navigation,
   cancellation and exact export through EngineClient. Verify terminal cleanup.
5. Complete native hard cancellation and file-dialog graph workflows; launch the
   real desktop app and test local/offline/remote workflows, then build/install
   macOS artifacts and exercise uninstall/reinstall. Use native CI machines for
   Windows/Linux and distinguish compilation from actual launch evidence.
6. Run measured performance/security/installer checks and all applicable release
   gates; fix failures. Review cargo-dist, publish order and packaging without
   adding accidental publishing. Check published version history before RC naming.
7. Keep F-01–F-07 regression checks in the build/release gates, update final
   docs/evidence and repeat the full release-readiness assessment.

## Release recommendation

**Internal alpha only**, with the limitations above. **Not ready for public beta,
release candidate or stable public release**. The graph foundation and initial
browser workflow have real passing evidence, but mandatory TUI/persistence/editor/
local-cancellation work and installer/platform verification are still open.
