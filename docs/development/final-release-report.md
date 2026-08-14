# Final release report

!!! warning "Superseded report"
    This report describes an earlier macOS-hosted consolidation and is retained
    only as historical evidence. It is not the current release verification.
    The authoritative result for this task is
    `final-linux-release-report.md`, generated only after the Linux x86_64
    Docker release gate passes.

## Repository

- Final working-tree base commit: `112a61d9e5f559405c0804bbea91e5bb2582c8f6`.
  The release-consolidation changes are present in the working tree and were not
  committed by this task, so no later commit hash is claimed.
- Version: `0.1.0`, sourced from `[workspace.package]`.
- Rust: `rustc 1.96.1 (31fca3adb 2026-06-26) (Homebrew)`.
- Validation host: macOS, arm64.

## Architecture

`rxchef::execution` is the shared byte-oriented execution engine. CLI run,
pipe/bake, saved recipes and pipelines, project execution, the TUI, Magic, and
the JSONL server delegate to it. It resolves operations through the generated
registry and enforces step/output limits and flow-control semantics. Store code
is isolated in `rxchef_store`; public Rust integration is exposed through
`rxchef::catalog` and `rxchef::execute`. The C ABI is explicitly experimental.

## CLI

Top-level commands are `operations`, `operation describe`, `list`, `info`,
`run`, `pipe`, `recipe`, `bake`, `pipeline`, `var`, `history`, `magic`, `scan`,
`project`, `serve --stdio`, `completions`, and `manpage`. Clap usage failures
exit 2; invalid command input/lookup exits 3; execution exits 4; store or I/O
failures exit 5; unavailable features exit 6. Successful payloads remain on
stdout and diagnostics remain on stderr.

## Server

The persistent newline-delimited protocol is version 1. Methods are `ping`,
`operations`, `describe`, `run`, `bake`, and `shutdown`. It accepts JSON-RPC 2.0
and the documented compact request form, transports binary data as Base64,
supports notifications, bounds request frames, recovers after malformed or
oversized requests, and terminates cleanly on EOF or shutdown.

## Library

Stable public entry points are `rxchef::catalog::{list, describe}` and
`rxchef::execute::{run, bake}` with structured errors. The lower-level
`rxchef::execution::{execute, ExecutionRequest, ExecutionOptions,
ExecutionOutcome}` API supports variables, tracing, flow control, fuel, and
output limits. All interfaces preserve arbitrary bytes.

## Operations

The generated audit records:

| Classification | Count |
|---|---:|
| Total | 478 |
| Complete | 0 |
| Feature-gated in the default build | 8 |
| Partial | 470 |
| Unsupported | 0 |
| Successful implementation placeholders | 0 |

`complete = 0` is intentional: existing implementations were not promoted
without operation-by-operation parity evidence. All registered operations have
metadata, mapped tests, generated documentation, a benchmark case or explicit
skip reason, and an honest parity/status value. The eight gated operations name
their `pgp`, `jsonata`, or `tesseract` requirement.

## Tests

The default final workspace inventory contains 1,867 passing Rust tests and no
ignored tests: 1,750 operation tests, 12 pipeline integration tests, 47 Core
unit tests, 6 CLI unit tests, 19 CLI integration tests, 26 server conformance
tests, 2 Store unit tests, and 5 doctests. The operation audit detects 338
known-answer-style mappings, one differential marker, and one property marker.
Three bounded libFuzzer targets cover recipes/execution, Magic, and streaming
scan; CI compiles every target. These text-based audit counts are evidence
inventory, not inflated claims that every test is an independent standard KAT.

## Bugs fixed

- Unified divergent execution and flow-control paths and added fuel/output caps.
- Made CLI output binary-safe, atomic for files, versioned for JSON, and clean
  on broken pipes.
- Removed implicit hex guessing and added checked argument conversions and
  normalized-name collision detection.
- Prevented implicit project-store creation and cross-scope mutations; made
  writes atomic and secret/history handling metadata-aware.
- Removed source-tree writes from normal Cargo builds and made registry drift a
  deterministic gate.
- Replaced ignored/no-op evidence in HMAC, HASSH, LZString, LZNT1, DoH, image,
  MD2, Rabbit, Salsa, XSalsa, RC6, and macro-doctest coverage.
- Bounded server requests, DoH responses, Magic search, Scanner token retention,
  and Scanner finding counts.
- Hardened C pointer/length/ownership checks and shipped a matching header.
- Removed 485 stale hand-maintained version headers from Rust source files.

## Benchmarks

Representative quick-suite measurements were historically written to
`docs/_generated/benchmarks.json`; current measurements live under
`benchmarks/results/`. Cases are declared in `benchmarks/cases.toml`. Run
`cargo xtask bench-docs --quick` for the small set or `cargo xtask bench-docs
--full` for the expanded set. Results include commit,
toolchain, OS, architecture, profile, median, and p95 and are reference values,
not hardware-independent guarantees.

## Documentation

Operation pages and the quality matrix are generated from runtime metadata.
Build the site with `mkdocs build --strict`; check generated content with
`cargo xtask docs --check` and
`cargo run --example generate_operation_docs -- --check`.

## Historical CI configuration

At the time of this historical report, CI defined formatting, scoped Clippy correctness/suspicious lints, Unix
workspace builds/tests, Windows Core/CLI/Store builds/tests, Linux all-feature
tests, registry/audit/docs drift gates, strict MkDocs, fuzz-target compilation,
an installed-binary smoke test, and informational coverage. Those build/test
workflows have since been removed and are not a release gate.

## Known Limitations

- 470 operations remain conservatively `partial` and all parity values remain
  unpromoted until independently verified; the generated matrix is authoritative.
- Fuzz targets compile, but no time-bounded fuzz campaign or corpus coverage
  result is claimed in this local run.
- The full benchmark profile and target-specific release archives were not
  produced on this single macOS host.
- Windows was not verified by the historical local run.
- `crates/cli/src/main.rs` remains large; command behavior is consolidated, but
  further file-level decomposition is maintainability work.
- The root lockfile still contains yanked upstream `spin 0.9.8`; the locked
  build passes, but the dependency should be upgraded deliberately.
- `cargo package` succeeds for Core and Store. CLI packaging requires Core and
  Store 0.1.0 to be published first, so publication order is Core → Store → CLI.
- The FFI surface is experimental rather than covered by the stable Rust API
  compatibility promise.

## Reproduction

```bash
cargo fmt --all -- --check
cargo metadata --format-version 1 --no-deps
cargo check --workspace --all-targets --all-features
cargo build --workspace --all-targets
cargo test --workspace
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- \
  -D clippy::correctness -D clippy::suspicious
cargo xtask check-registry
cargo xtask audit-operations
cargo xtask docs --check
cargo run --example generate_operation_docs -- --check
cargo check --manifest-path fuzz/Cargo.toml --bins
mkdocs build --strict
cargo install --path crates/cli --locked --force
rxchef --version
rxchef --help
rxchef operations --json
```

The release smoke test additionally decodes a Base64-wrapped binary fixture and
compares the restored file byte-for-byte.
