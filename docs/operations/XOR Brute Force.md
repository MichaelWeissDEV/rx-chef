# XOR Brute Force

## Overview

Enumerate all possible XOR solutions. Optionally enter a string that you expect to find in the plaintext to filter results (crib).

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
| 1 | Key length | `UnsignedInteger` | no | `1` | — | no | Length of the XOR key in bytes (1..=2 recommended) |
| 2 | Sample length | `UnsignedInteger` | no | `100` | — | no | Number of bytes of input to process |
| 3 | Sample offset | `Integer` | no | `0` | — | no | Byte offset to start sampling from |
| 4 | Scheme | `String` | no | `Standard` | — | no | Standard, Input differential, or Output differential |
| 5 | Null preserving | `Boolean` | no | `false` | — | no | Do not XOR null bytes or bytes equal to the key |
| 6 | Print key | `Boolean` | no | `true` | — | no | Prefix each result with the key used |
| 7 | Output as hex | `Boolean` | no | `false` | — | no | Output results as hex instead of text |
| 8 | Crib (known plaintext string) | `String` | no | `<empty>` | — | no | Filter results to those containing this string |

## Implementation

The implementation is in `src/operations/xor_brute_force.rs` and declares `Bytes` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "XOR Brute Force" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/xor_brute_force.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
