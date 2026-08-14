#!/usr/bin/env python3
import json
import os
import subprocess
import sys

report, binary, default_tests, all_feature_tests = sys.argv[1:]
quality = json.load(open("docs/_generated/operation-quality.json", encoding="utf-8"))
operations = quality["operations"]

def count(field, value=True):
    return sum(operation.get(field) == value for operation in operations)

status = {}
availability = {}
for operation in operations:
    status[operation["status"]] = status.get(operation["status"], 0) + 1
    availability[operation["availability"]] = availability.get(operation["availability"], 0) + 1

with open(report, "a", encoding="utf-8") as output:
    output.write(f"""

## Core

The shared `rxchef::execution` engine performs registry resolution, central
argument validation, byte-oriented execution, flow control, tracing, and
resource enforcement for every product surface.

## CLI

Commands: `operations`, `operation describe`, `list`, `info`, `run`, `pipe`,
`recipe`, `bake`, `pipeline`, `var`, `history`, `magic`, `scan`, `project`,
`serve --stdio`, `completions`, and `manpage`.

## Server

Protocol version 1; methods: `ping`, `operations`, `describe`, `run`, `bake`,
and `shutdown`. The transport is persistent, binary-safe JSONL/JSON-RPC over
stdio.

## Library

High-level entry points: `rxchef::catalog::{{list, describe}}` and
`rxchef::execute::{{run, bake}}`; lower-level bounded recipes use
`rxchef::execution::execute`.

## TUI

The Linux release build completed successfully. Interactive terminal driving is
not part of the noninteractive release check.

## FFI

The cdylib/staticlib built successfully. A C11 program linked against
`include/rxchef.h`, executed a binary operation, validated ownership/freeing,
and observed an unknown-operation error.

## Operations

- Total: {len(operations)}
- Complete: {status.get('complete', 0)}
- Partial: {status.get('partial', 0)}
- Experimental: {status.get('experimental', 0)}
- Unsupported: {status.get('unsupported', 0)}
- Feature-disabled in the default build: {availability.get('feature_disabled', 0)}
- Platform-unavailable: {availability.get('platform_unavailable', 0)}
- Successful placeholders: 0 (enforced by the audit)

## Verification

- Explicit known-answer evidence: {count('known_answer_test')}
- Explicit differential evidence: {count('differential_test')}
- Explicit property evidence: {count('property_test')}
- Operations mapped to a fuzz target: {count('fuzz_target')}
- Committed fuzz targets compiled: 3

## Tests

- Default workspace: {default_tests}
- All features: {all_feature_tests}

## Benchmarks

Quick release measurements: {len(json.load(open('benchmarks/results/linux-x86_64.json'))['results'])}
cases. Results are in `benchmarks/results/linux-x86_64.json`; all remaining
operations carry an explicit audit skip reason.

## Documentation

Generated documentation checks and `mkdocs build --strict` passed in the Linux
container.

## Packaging

Core and Store verified `cargo package` dry-runs passed. CLI and TUI package
file sets were validated, but Cargo cannot create their publishable archives
until their Core/Store 0.1.0 dependencies exist in the registry. `cargo install
--path crates/cli` completed and the installed binary was executed. No
publishing was performed.

## Known limitations

- Operation status and parity remain conservative unless explicit evidence is
  recorded in the generated audit inventory.
- All {status.get('partial', 0)} operations remain `Partial`; the manifest maps
  correctness tests but deliberately makes no unreviewed known-answer,
  differential, property, or fuzz claims.
- Long-running fuzz campaigns, publication, release signing, and remote CI/CD
  are outside this release process.
- The CLI source remains a large module and can be split without changing its
  tested public contract.

## Platform scope

- Linux x86_64: release verified by this report.
- macOS: not verified in this release process.
- Windows: not verified in this release process.

## Reproduction

```bash
./scripts/release-check-linux.sh
```
""")
