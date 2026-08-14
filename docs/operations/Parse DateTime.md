# Parse DateTime

## Overview

Parses a DateTime string using strftime format specifiers and displays detailed date/time information including day of year, week number, quarter, and leap year status. Format uses strftime tokens (e.g. %d/%m/%Y %H:%M:%S).

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

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Input format string | `String` | no | `%d/%m/%Y %H:%M:%S` | — | no | strftime format string (e.g. %d/%m/%Y %H:%M:%S) |
| 2 | Input timezone | `String` | no | `UTC` | — | no | Timezone name (currently UTC only) |

## How it works

Parses a DateTime string using strftime format specifiers and displays detailed date/time information including day of year, week number, quarter, and leap year status. Format uses strftime tokens (e.g. %d/%m/%Y %H:%M:%S).

## Implementation

The implementation is in `src/operations/parse_date_time.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Parse DateTime"
```

For file or binary input use `rxchef run "Parse DateTime" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Parse DateTime" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/parse_date_time.rs

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
