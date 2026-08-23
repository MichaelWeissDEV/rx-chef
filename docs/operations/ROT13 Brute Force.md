# ROT13 Brute Force

## Overview

Tries all meaningful Caesar rotation amounts, with optional character classes, sampling and a known-plaintext filter.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
| Input requirement | `Required` |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `Bytes`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Rotate lower case chars | `Boolean` | no | `true` | — | no | Rotate ASCII lower-case letters |
| 2 | Rotate upper case chars | `Boolean` | no | `true` | — | no | Rotate ASCII upper-case letters |
| 3 | Rotate numbers | `Boolean` | no | `false` | — | no | Rotate ASCII decimal digits |
| 4 | Sample length | `UnsignedInteger` | no | `100` | — | no | Maximum number of input bytes to rotate |
| 5 | Sample offset | `UnsignedInteger` | no | `0` | — | no | Byte offset at which the sample begins |
| 6 | Print amount | `Boolean` | no | `true` | — | no | Prefix each result with its rotation amount |
| 7 | Crib (known plaintext string) | `String` | no | `<empty>` | — | no | Only retain rotations containing this text, case-insensitively |

## Implementation

The implementation is in `src/operations/rot13_brute_force.rs` and declares `Bytes` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "ROT13 Brute Force" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `Bytes` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/rot13_brute_force.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
