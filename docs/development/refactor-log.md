# Release consolidation log

This log records observed changes during the pre-1.0 release consolidation. It
does not present planned work as completed work.

Entries about GitHub Actions and cross-platform jobs are historical. The
current release process verifies Linux x86_64 locally in Docker; macOS and
Windows are not verified, and GitHub CI/CD is not a release gate.

## Phase A — baseline

### What changed

- Recorded the clean-checkout environment and all required baseline gates in
  `baseline.md`.
- Ran the mandatory placeholder and weak-assertion searches.

### Why

The release work needs a reproducible starting point and an honest inventory of
pre-existing defects.

### Tests added

None; Phase A intentionally measured the unmodified code.

### Bugs found

- Six PGP-related implementations explicitly identify themselves as stubs.
- Three HASSH client tests accept either success or failure and therefore prove
  nothing.
- Seven operation tests were ignored in the baseline suite.
- The normal build still uses `build.rs` to write `src/operations/mod.rs`.

### Remaining work

See the later phases below and `baseline.md` for the exact gate results.

## Phase B — CLI command tree

### What changed

- Verified that every `Command` variant, including `History`, has exactly one
  arm in the exhaustive central dispatcher.
- Verified the real Clap tree contains `operations`, `operation describe`,
  `bake`, and `serve --stdio`.
- Extended the real-binary CLI test to execute `--help` for every declared
  top-level command.

### Why

An exhaustive Rust match protects the dispatcher at compile time; exercising
each command through the installed command tree additionally catches Clap/API
drift.

### Tests added

- The existing top-level help conformance test now verifies every command's
  command-specific help path. `cargo test -p rxchef_cli --test cli` passes all
  10 tests.

### Bugs found

No missing Phase-B dispatch arm existed at the baseline commit. At that phase
boundary, shell completions and `project init` were still absent; they were
implemented in later Store and Packaging phases.

### Remaining work

- Split the monolithic CLI during the error/input/output consolidation.
- Add the remaining final command surface only when its backing behavior is
  real and tested.

## Phase C — shared execution engine

### What changed

- Added `rxchef::execution` with `ExecutionRequest`, `ExecutionOptions`,
  `ExecutionOutcome`, `Recipe`, `RecipeStep`, `VariableContext`, structured
  `ExecutionError`, and metadata-only `TraceEntry` values.
- Moved all Recipe and Flow-Control interpretation out of the integration
  transport and into the Core execution engine.
- Routed CLI `run`, linear `pipe`, Flow-Control `pipe`, Recipe execution,
  saved-pipeline execution, project execution, server `run`/`bake`, Magic
  candidate execution, and TUI operation execution through the same engine.
- Removed the CLI's `all_bytes` retention of every intermediate pipeline value.
- Changed CLI trace rendering from payload dumps to byte counts and elapsed
  time.
- Kept unexpanded arguments in new History records so variable values are not
  copied into argument history.

### Why

Previously, the CLI had separate linear and Flow-Control executors and the TUI
called the operation runtime directly. That allowed frontend-dependent
semantics and retained unnecessary copies of large intermediate outputs.

### Tests added

- Core tests cover linear and Flow-Control recipes, trace metadata, step fuel,
  and output-size limits.
- A real-binary conformance test proves identical Flow-Control bytes from the
  Rust API, CLI `pipe`, CLI `bake`, and the persistent stdio server.
- After the refactor: 37 Core tests, 11 CLI conformance tests, and 25 server
  conformance tests pass; the TUI target builds and its empty smoke suite
  completes.

### Bugs found

- The legacy public `Pipeline` type still owns a typed-operation execution loop.
  It remains for compatibility and must be migrated or explicitly deprecated
  before the hard single-engine gate is claimed complete.
- The legacy `recipe` CLI path imports a file as a side effect before falling
  back to execution-only loading.

### Remaining work

- Make the new engine errors source-preserving down to `OperationError` rather
  than carrying the legacy runtime message at the innermost boundary.
- Finish the legacy `Pipeline` migration and remove the recipe import side
  effect.

## Phase D — input, output, and type contracts

### What changed

- Non-TTY `auto` output now always writes exact bytes; terminal-only binary
  previews no longer corrupt redirected data.
- Added `--format auto|raw|text|hex|base64|json` and atomic `--output-file` to
  `run`, `pipe`, `recipe`, and `bake`, retaining `--hex`/`--json` aliases.
- Versioned CLI JSON output and added `output_is_utf8`; the server result now
  also distinguishes exact UTF-8 from its lossy convenience field.
- Added clean Broken-Pipe handling to operation catalog output.
- Added strict declared-output validation for text, JSON, and numbers.
- Rejected non-finite `num:` values and made legacy `f64` to integer accessors
  reject NaN, infinity, fractions, negatives for unsigned values, and values
  outside the target range.

### Why

The previous `auto` path converted invalid UTF-8 to a textual hex dump even
when stdout was a pipe. Declared JSON/text output could also silently become
untyped bytes, and Rust float casts silently saturated or truncated integer
arguments.

### Tests added

- Exact binary redirect for `00 01 02 ff fe fd 80 00`.
- All explicit output representations, invalid UTF-8 text rejection, atomic
  output-file replacement, JSON schema fields, and downstream pipe closure.
- Strict output-contract and checked-integer unit tests.
- The full `cargo test --workspace` suite remains green after strict validation.

### Bugs found

- `Utils::convert_to_byte_array` still auto-detects unprefixed hex strings. It
  cannot be removed safely until argument kinds are available to distinguish
  textual and byte-valued legacy parameters.
- The CLI still has a string-based top-level error layer and does not yet expose
  stable categorized exit codes.

### Remaining work

- Complete the argument metadata/resolver migration, then remove implicit hex
  detection.
- Introduce the categorized CLI error layer and apply the shared output options
  to saved-pipeline execution.

## Phase E — operation metadata and argument validation

### What changed

- Added conservative operation status, parity, input, side-effect, feature,
  determinism, and argument-kind metadata to the public descriptors.
- Added normalized identifiers and collision tests, strict named/positional
  resolution, legacy numeric/bool validation, and explicit byte prefixes.
- Removed implicit hexadecimal guessing for unprefixed strings.

### Why

Machine consumers and validation need honest, stable metadata. Unverified
operations remain `partial` with `unknown` parity rather than receiving inferred
release claims.

### Tests added

- Registry identifier and argument-name collision checks.
- Descriptor/filter CLI tests and strict argument conversion tests.
- Plain `deadbeef` versus explicit `hex:deadbeef` coverage.

### Bugs found

- Fang URL exposed two punctuation-only argument names that normalized to the
  same key; they now have distinct descriptive names.
- XSalsa tests depended on undocumented automatic hexadecimal guessing.

### Remaining work

- Replace conservative legacy inference with explicit metadata as operations
  are audited batch by batch.

## Phase F — store, projects, variables, and history

### What changed

- Added ancestor project discovery, explicit `project init`, `RXCHEF_HOME`, and
  project/global default-scope selection.
- Split resolved recipe reads from exact-scope mutation reads, and made recipe
  file execution read-only.
- Added atomic Store writes and owner-only Unix permissions for variables and
  History.
- Added secret variables, stdin input, redacted listings, Unicode-safe previews,
  and sensitive argument redaction in History.
- History replay now requires fresh full input instead of executing a preview.

### Why

The old behavior could create projects implicitly, mutate a global pipeline
using a same-named project definition, leak variables in routine output, and
silently replay truncated data.

### Tests added

- Parent-directory project discovery and Unicode-safe preview unit tests.
- Isolated real-binary tests with temporary `RXCHEF_HOME` cover initialization,
  default/override scopes, secrets, collision-safe mutations, and read-only file
  recipes.
- Unit coverage checks schema-driven History redaction.

### Bugs found

- Variable previews sliced UTF-8 text at byte indices and could panic.
- Project mutations silently created `.rxchef`; recipe execution imported files.

### Remaining work

- Store locking for concurrent writers remains future work.

## Phase G — deterministic operation registry

### What changed

- Removed the source-writing root `build.rs` and its build dependency.
- Added `cargo xtask generate-registry` and `cargo xtask check-registry` with
  sorted discovery, normalized module collision detection, and an explicit
  helper-module allowlist.
- Replaced the CI post-build diff heuristic with the standalone registry check.

### Why

A normal Cargo build must be read-only with respect to committed source. An
explicit generator also makes drift and malformed operation modules fail with a
direct diagnostic.

### Tests added

- `cargo xtask check-registry` renders to `target/xtask`, formats the result,
  and compares it byte-for-byte with the committed registry.
- A workspace build left the committed registry hash unchanged.

### Bugs found

- Three source modules are helpers rather than unit-struct operations; they are
  now documented in the generator's explicit allowlist instead of being
  silently skipped.

### Remaining work

- The operation audit in Phase H must build on the registry and verify deeper
  metadata/implementation invariants.

## Phase H — operation quality audit

### What changed

- Added `cargo xtask audit-operations` and generated a 478-row JSON inventory
  plus a human-readable matrix.
- The audit checks registry name/ID uniqueness, metadata, source/test/docs
  mappings, argument documentation, benchmark coverage or skip rationale,
  placeholder macros/markers, and feature-gated status consistency.
- Generated registry entries now expose their source module to tooling without
  filename guessing.

### Why

Operation quality must be visible and machine-checkable. The matrix keeps
unknown parity and absent KAT/property/fuzz coverage visible instead of treating
the existence of a test file as proof of completeness.

### Tests added

- The audit itself is a CI gate and emits deterministic artifacts at
  `docs/_generated/operation-quality.json` and
  `docs/reference/operation-matrix.md`.

### Bugs found

- Six PGP operations still described their real feature-gated Sequoia backend
  as a stub. The stale placeholder wording was removed; their unavailable
  minimal-build status remains explicit.

### Remaining work

- Phase I must convert the matrix's conservative `partial`/`unknown` and absent
  KAT/property/fuzz cells only when corresponding evidence is added.

## Phase I — operation verification batches

### What changed

- Removed every ignored operation test and every assertion that accepted both
  success and failure.
- Made HMAC key encodings explicit and restored the RFC 2202 `Jefe` vector.
- Added a constructed SSH KEXINIT HASSH KAT, strict hex parsing, valid LZString
  vectors with resource caps, valid/malformed LZNT1 fixtures, and local-only DoH
  tests with network limits.
- Removed MD2's ineffective configurable-rounds argument and updated legacy
  Rabbit/Salsa vectors to use explicit byte encodings.

### Why

Ignored and tautological tests hid five real regressions after implicit hex
guessing was removed, plus an HMAC ambiguity and invalid compression fixtures.

### Tests added

- The operation suite now executes 1,750 tests with zero ignored tests.
- DNS tests bind only an ephemeral loopback listener and require no public
  network service.

### Bugs found

- Literal HMAC key `Jefe` was silently decoded as Base64.
- HASSH discarded invalid hex characters instead of rejecting them.
- LZString decoding used repeated character scans and had no expansion cap.
- MD2 documented a rounds setting the implementation could not support.

### Remaining work

- The generated matrix deliberately keeps operations `partial`/`unknown` where
  independent upstream parity evidence is absent; no blanket completeness claim
  is made from passing local tests alone.

## Phase J — server, public API, and FFI boundary

### What changed

- Added bounded JSONL framing with a configurable 1 MiB default and recoverable
  `-32004` resource-limit responses.
- Added stable `rxchef::catalog` and `rxchef::execute` entry points with
  structured public errors and compiling Rustdoc examples.
- Added FFI null/length and finite-number checks, binary ownership tests, and a
  checked-in C header.
- Explicitly marked the C ABI experimental instead of promising 1.0 ABI
  stability that the current opaque argument surface cannot yet guarantee.

### Why

`BufRead::lines` can allocate an unbounded request, and Rust consumers
previously had to use string-error transport helpers. The existing C ABI also
accepted inconsistent null/length pairs.

### Tests added

- Exact request-boundary acceptance, oversized-line rejection, post-error
  recovery, and a real-server configurable-limit test.
- FFI tests cover null/length mismatches, non-finite numbers, exact binary
  output, and matching deallocation.

### Bugs found

- The server had no input framing limit.
- FFI treated null input with a positive length as empty input.

### Remaining work

- ABI stabilization and generated-header drift checking are deferred while the
  C API is explicitly experimental; stdio and Rust are the stable integration
  surfaces.

## Phase K — benchmark catalog and measurements

### What changed

- Added `cargo xtask bench-docs --quick|--full`, which always relaunches the
  harness in release mode and writes schema-versioned environment metadata,
  median, p95, spread, and throughput values.
- Added the representative benchmark catalog at `benchmarks/cases.toml`.

### Why

Debug timings and context-free numbers are misleading. Generated benchmark
data now identifies commit, toolchain, OS, architecture, profile, and suite and
states that it is not a runtime guarantee.

### Tests added

- Ran the quick suite in release mode and generated
  `docs/_generated/benchmarks.json` from actual measurements.

### Bugs found

- The old JSON omitted p95, commit, toolchain, platform, and a case identifier.

### Remaining work

- The full suite is intentionally more expensive and is suitable for scheduled
  release runs rather than the fast PR gate.

## Phase L — generated documentation

### What changed

- Expanded all 478 generated operation pages to the required status, type,
  argument, execution, example, compatibility, security, testing, performance,
  limitation, and reference sections.
- Added the baseline/refactor log to navigation and regenerated both operation
  reference artifacts.

### Why

The earlier pages were metadata stubs and did not surface honest partial/unknown
status or verification gaps.

### Tests added

- Both documentation generators pass their no-write `--check` modes.
- `mkdocs build --strict` completes without warnings.

### Bugs found

- The two development audit pages existed outside navigation.

### Remaining work

- Rich algorithm-specific sidecars can improve prose without duplicating the
  generated factual schema.

## Phase M — cross-platform CI

### What changed

- Added Windows Core/CLI/Store build and test coverage alongside Linux/macOS.
- Added Linux all-feature tests, post-generation diff, install smoke, and
  informational llvm-cov jobs.
- Declared Rust 1.96 as the tested MSRV for this release line.

### Why

The previous CI omitted Windows, all-feature tests, installation, and coverage.

### Tests added

- CI definitions now exercise the real installed binary and generated gates.

### Remaining work

- Rust 1.96 is the measured toolchain floor, not a claim that older compilers
  were exhaustively bisected.

## Phase N — packaging and release surface

### What changed

- Added Bash, Zsh, Fish, and PowerShell completion generation plus `rxchef(1)`
  manpage generation directly from Clap.
- Updated installation, release, README, status, and changelog documentation;
  clarified stable Rust/stdio versus experimental C integration.
- Added a CI install-smoke job and documented release archive/checksum/smoke
  requirements.

### Why

Release artifacts must derive command syntax from the actual CLI and package
claims must match tested interfaces.

### Tests added

- Real-binary tests assert every completion and the manpage are non-empty.
- A locked `cargo install --path crates/cli` into an isolated root succeeded;
  the installed binary passed version, help, and catalog smoke commands.
- `cargo package --list` was inspected for Core, Store, and CLI.

### Bugs found

- The locked graph contains upstream-yanked `spin 0.9.8`; Cargo warns but the
  locked install succeeds. This is recorded as a release dependency risk.

### Remaining work

- GitHub release archive production and target-specific smoke tests run only in
  release automation, not on this single macOS host.
- crates.io naming/publication status requires registry verification at actual
  publish time; packages were not renamed speculatively.

## Final release gate — 0.1.0

### What changed

- Centralized version, edition, license, and repository metadata under
  `[workspace.package]`, bumped the consolidated workspace to 0.1.0, and
  removed 485 stale version banners from Rust sources.
- Added stable CLI failure classes and exit codes, explicit Magic candidate and
  byte budgets, Scanner token/finding limits, and three bounded fuzz targets.
- Added complete feature requirement metadata for the eight default-build gated
  operations and made the audit reject missing feature names.
- Added `docs/development/final-release-report.md`.

### Verification

- `cargo fmt`, metadata, normal/all-feature checks, and workspace build pass.
- Default workspace: 1,867 passed, 0 failed, 0 ignored.
- All features: 1,867 passed, 0 failed, 0 ignored.
- Scoped Clippy correctness/suspicious lints pass; historical style warnings
  remain non-blocking and are not represented as fixed.
- Registry, 478-operation audit, both documentation drift gates, fuzz-target
  compilation, strict MkDocs, and `git diff --check` pass.
- An isolated locked release install produced `rxchef 0.1.0`; help/catalog
  smokes passed and an eight-byte non-UTF-8 Base64 roundtrip compared exactly.
- `cargo package` succeeds for Core and Store. CLI packaging correctly waits
  for the new Core version to exist in the registry.

### Remaining work

- Publish in dependency order (Core, Store, CLI), upgrade the yanked locked
  `spin 0.9.8` deliberately, execute target-specific release automation, and
  run sustained fuzz campaigns. These are not claimed by the local gate.
