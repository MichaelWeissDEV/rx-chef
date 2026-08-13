# DateTime Delta

## Overview

Calculates a new DateTime value given an input DateTime value and a time difference (delta) from the input DateTime value. Uses strftime format strings.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | available |
| Features | none |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `HTML`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Built in formats | `String` | no | `Standard date and time` | — | no | Common datetime formats |
| 2 | Input format string | `String` | no | `%d/%m/%Y %H:%M:%S` | — | no | strftime format string for parsing and formatting (e.g. %d/%m/%Y %H:%M:%S) |
| 3 | Time Operation | `String` | no | `Add` | — | no | Whether to add or subtract the delta |
| 4 | Days | `Integer` | no | `0` | — | no | Number of days |
| 5 | Hours | `Integer` | no | `0` | — | no | Number of hours |
| 6 | Minutes | `Integer` | no | `0` | — | no | Number of minutes |
| 7 | Seconds | `Integer` | no | `0` | — | no | Number of seconds |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/date_time_delta.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "DateTime Delta"
```

For file or binary input use `rxchef run "DateTime Delta" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "DateTime Delta" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

The mapped Rust test and available KAT/differential/property/fuzz evidence are recorded in the [operation quality matrix](../reference/operation-matrix.md).

## Performance

See [benchmark results](../performance/results.md). Operations outside the representative catalog are explicitly marked with a skip rationale in the machine-readable quality inventory. Measurements are hardware-dependent reference values, not guarantees.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
