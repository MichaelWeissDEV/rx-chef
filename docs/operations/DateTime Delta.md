# DateTime Delta

## Overview

Calculates a new DateTime value given an input DateTime value and a time difference (delta) from the input DateTime value. Uses strftime format strings.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | Available |
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

Calculates a new DateTime value given an input DateTime value and a time difference (delta) from the input DateTime value. Uses strftime format strings.

## Implementation

The implementation is in `src/operations/date_time_delta.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

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

Correctness:
- tests/tests/operations/date_time_delta.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Not measured. Reason: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
