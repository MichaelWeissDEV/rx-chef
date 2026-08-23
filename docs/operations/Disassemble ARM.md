# Disassemble ARM

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Disassembles ARM machine code into assembly language.<br><br>Supports ARM (32-bit), Thumb, and ARM64 (AArch64) architectures using the Capstone disassembly framework.<br><br>Input should be in hexadecimal.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | FeatureDisabled |
| Input requirement | `Required` |
| Features | disassembly |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Architecture | `String` | no | `ARM (32-bit)` | — | no | The ARM architecture to use. |
| 2 | Mode | `String` | no | `ARM` | — | no | The instruction set mode. |
| 3 | Endianness | `String` | no | `Little Endian` | — | no | The byte order. |
| 4 | Starting address (hex) | `Integer` | no | `0` | — | no | The address to start disassembling from. |
| 5 | Show instruction hex | `Boolean` | no | `true` | — | no | Whether to show the hex bytes of each instruction. |
| 6 | Show instruction position | `Boolean` | no | `true` | — | no | Whether to show the address of each instruction. |

## Implementation

The implementation is in `src/operations/disassemble_arm.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Disassemble ARM" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/disassemble_arm.rs

Known-answer tests:
- tests/tests/operations/disassemble_arm.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
