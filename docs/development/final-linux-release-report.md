# Linux release report — verification pending

!!! warning "Not release-verified"
    The current consolidated source tree has **not** completed the required
    Linux x86_64 Docker gate. Docker execution was explicitly stopped before
    the final run. This report records completed host evidence and the exact
    missing release gate; it is not a Linux release certificate.

## Commit

- Base commit: `fe367ee2ef145a5745ab27a93fea42d6a38dec09`
- Evaluated tree: that commit plus the uncommitted consolidation changes
- Report date: `2026-08-14`

Because the evaluated tree contains uncommitted changes, the commit alone does
not reproduce it. A final report generated after a clean Linux gate must record
the committed tree that was actually tested.

## Version

`0.1.0` (pre-1.0 API and recipe-schema stabilization)

## Environment

### Required final environment

| Item | Required value | Current status |
|---|---|---|
| Container base | `debian:bookworm-slim` | Defined, final run not executed |
| OS | Debian GNU/Linux 12 | Not verified for the current tree |
| Architecture | `x86_64` | Not verified for the current tree |
| Rust | stable via rustup | Not recorded for a final current-tree run |
| Cargo | matching stable toolchain | Not recorded for a final current-tree run |

An earlier baseline at the same base commit passed on Debian 12 x86_64 with
`rustc 1.96.1` and `cargo 1.96.1`. It predates the final consolidation changes
and is retained separately in [Linux baseline](linux-baseline.md).

### Supplemental host environment

The allowed current-tree checks ran on macOS 26.4, ARM64, with Homebrew
`rustc 1.97.1` and `cargo 1.97.1`. These results are useful development
evidence but do not substitute for Linux x86_64 release verification.

## Core

All frontends use the shared byte-oriented `rxchef::execution` engine for
registry resolution, explicit argument-schema validation, recipes, flow
control, tracing, and resource limits. Implementation status and build
availability are separate metadata fields. Runtime failures use structured
unknown-operation, unavailable, invalid-argument, operation, and
output-validation variants. Missing input remains distinct from explicitly
supplied empty input.

## CLI

Release build and host smokes passed for `operations`, `operation describe`,
`list`, `info`, `run`, `pipe`, `recipe`, `bake`, `pipeline`, `var`, `history`,
`magic`, `scan`, `project`, `serve`, `completions`, and `manpage`. The release
binary reported `rxchef 0.1.0`; help, the 478-entry all-operation catalog,
descriptor lookup, Base64 decoding, exact binary roundtrip, and the versioned
recipe fixture passed on the supplemental host.

## Server

Protocol version 1 supports `ping`, `operations`, `describe`, `run`, `bake`,
and `shutdown` over persistent JSONL/JSON-RPC 2.0 stdio. The host conformance
smoke passed multiple requests, binary-safe responses, error handling, and
shutdown behavior.

## Library

High-level APIs are `rxchef::catalog::{list, describe}` and
`rxchef::execute::{run, bake}`. Typed pipelines and the lower-level bounded
`rxchef::execution::execute` API remain available. APIs preserve exact bytes
and do not depend on terminal behavior.

## TUI

The optimized TUI built successfully on the supplemental host. It uses the
shared recipe engine, Unicode-safe editing, and central history redaction.
Interactive terminal behavior and a current Linux build remain outside the
completed evidence.

## FFI

The static and dynamic libraries built on the supplemental host. A C11 smoke
program compiled and linked against `include/rxchef.h`, executed an operation
with binary input, freed its returned allocation, and observed a structured
unknown-operation error. Rust tests also cover invalid pointer/length pairs,
missing versus empty input, binary output, multiple allocations, operation
errors, and panic containment. Current-tree Linux linking is still pending.

## Operations

| Classification | Count |
|---|---:|
| Total | 478 |
| Complete | 0 |
| Partial | 478 |
| Experimental | 0 |
| Unsupported | 0 |
| Available in the default build | 467 |
| Feature-disabled in the default build | 11 |
| Platform-unavailable | 0 |
| Successful placeholders | 0 |

All 478 operations have explicit argument metadata, a correctness-test mapping,
documentation, and either benchmark evidence or an explicit skip reason. All
parity claims remain `unknown`; high test volume alone was not used to promote
any operation to `Complete`.

## Verification

| Explicit reviewed evidence | Operations |
|---|---:|
| Correctness-test mapping | 478 |
| Known-answer | 0 |
| Differential | 0 |
| Property | 0 |
| Fuzz-target mapping | 0 |

Three committed fuzz targets compile on the supplemental host. No long-running
fuzz campaign was performed, and compilation is not counted as operation-level
fuzz evidence.

## Tests

The current tree passed the following supplemental host suites:

| Suite | Default | All features |
|---|---:|---:|
| Operation tests | 1,704 | 1,749 |
| Pipeline integration | 12 | 12 |
| Core | 51 | 52 |
| CLI unit | 7 | 7 |
| CLI integration | 19 | 19 |
| Server | 26 | 26 |
| Store | 4 | 4 |
| TUI | 1 | 1 |
| Rustdoc | 5 | 5 |
| **Total** | **1,829** | **1,875** |

Workspace checks, all-target checks, all-feature checks, scoped Clippy
correctness/suspicious lints, registry validation, and operation audit also
passed on the host. These counts are not Linux results.

## Benchmarks

The explicit operation manifest records benchmark evidence for 5 operations;
the other 473 entries contain an explicit N/A/skip reason. The supplemental
full host suite measured 11 representative cases and wrote
`benchmarks/results/host-unverified.json`. No current-tree Linux benchmark
artifact is claimed.

## Documentation

Generated documentation drift checks, the generated operation reference check,
and `mkdocs build --strict` passed on the supplemental host. All 478 operation
pages derive machine facts from runtime metadata and preserve unknown or
unmeasured evidence as such.

## Packaging

- `cargo package -p rxchef --allow-dirty`: passed on the host.
- `cargo package -p rxchef_store --allow-dirty`: passed on the host.
- CLI/TUI package file lists: passed; archive verification requires published
  Core/Store `0.1.0` dependencies.
- `cargo install --path crates/cli --root /tmp/rxchef-host-install --force --locked`:
  passed, and the installed binary reported version `0.1.0`.
- No crate was published and no release archive was signed.

## Known limitations

- The mandatory current-tree Linux x86_64 Docker gate has not run; this is the
  only authority that can change this report to release-verified.
- The evaluated tree is dirty and therefore is not reproducible from the base
  commit alone.
- All operations remain `Partial`, all parity statuses remain unknown, and no
  reviewed known-answer, differential, property, or operation-level fuzz
  evidence is currently recorded.
- The locked dependency graph includes the upstream-yanked `spin 0.9.8`.
  Locked builds remain reproducible, but it should be updated before publishing.
- CLI and TUI publishable archives depend on publishing Core/Store `0.1.0`
  first.
- Long-running fuzzing, release signing, publication, CI/CD, macOS release
  verification, and Windows verification are outside the completed scope.

## Platform scope

- Linux x86_64: **not release-verified for the current tree**.
- macOS ARM64: supplemental host checks passed; **not release-verified**.
- Windows: **not verified**.

## Reproduction

The single required final gate remains:

```bash
./scripts/release-check-linux.sh
```

Only a fully successful run of that command on the committed current tree may
replace the status above with “Linux x86_64: release verified”.
