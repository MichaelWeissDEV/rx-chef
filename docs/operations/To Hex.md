# To Hex

## Overview

Converts the input string to hexadecimal bytes separated by the specified delimiter.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Exact` |
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
| 1 | Delimiter | `Enum` | no | `Space` | Space, Comma, Semi-colon, Colon, Line feed, CRLF, None, 0x with comma, \\x | no | Delimiter between hex bytes (Space, Comma, Semi-colon, Colon, Line feed, CRLF, None, 0x with comma, \\x) |
| 2 | Bytes per line | `UnsignedInteger` | no | `0` | — | no | Number of bytes per output line (0 = no wrapping) |

## Implementation

The implementation is in `src/operations/to_hex.rs` and declares `Bytes` input and `String` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "To Hex" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `Bytes` value is valid. Its `String` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Exact`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/to_hex.rs
- tests/tests/negative_contracts.rs

Known-answer tests:
- tests/tests/known_answer_vectors.rs

Differential tests:
- tests/tests/differential.rs

## Performance classification

Benchmark evidence:
- representative release benchmark

See [benchmark results](../performance/results.md) for measured environment and statistics.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
