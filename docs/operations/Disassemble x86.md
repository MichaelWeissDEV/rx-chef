# Disassemble x86

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Disassembly is the process of translating machine language into assembly language.<br><br>This operation supports 64-bit, 32-bit and 16-bit code written for Intel or AMD x86 processors. It is particularly useful for reverse engineering shellcode.<br><br>Input should be in hexadecimal.

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
| 1 | Bit mode | `Enum` | no | `64` | 64, 32, 16 | no | The bit mode (64, 32, 16) |
| 2 | Compatibility | `String` | no | `Full x86 architecture` | — | no | The compatibility mode (not all supported by Capstone) |
| 3 | Code Segment (CS) | `Integer` | no | `0` | — | no | The code segment (not used by Capstone) |
| 4 | Offset (IP) | `Integer` | no | `0` | — | no | The instruction pointer offset |
| 5 | Show instruction hex | `Boolean` | no | `true` | — | no | Whether to show instruction hex |
| 6 | Show instruction position | `Boolean` | no | `true` | — | no | Whether to show instruction position |

## Implementation

The implementation is in `src/operations/disassemble_x86.rs` and declares `String` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Disassemble x86" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/disassemble_x86.rs

Known-answer tests:
- tests/tests/operations/disassemble_x86.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
