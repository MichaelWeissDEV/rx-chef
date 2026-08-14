# Translate DateTime Format

## Overview

Parses a datetime string in one format and re-writes it in another. Uses strftime/strptime format strings (e.g. %d/%m/%Y %H:%M:%S). Timezone names are noted but conversion uses UTC unless a numeric offset is embedded in the format.

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
| 1 | Input format string | `String` | no | `%d/%m/%Y %H:%M:%S` | — | no | strftime format string for parsing input (e.g. %d/%m/%Y %H:%M:%S) |
| 2 | Input timezone | `String` | no | `UTC` | — | no | Timezone of the input datetime (informational; UTC assumed unless offset in format) |
| 3 | Output format string | `String` | no | `%A %d %B %Y %H:%M:%S` | — | no | strftime format string for the output (e.g. %A %d %B %Y %H:%M:%S %z) |
| 4 | Output timezone | `String` | no | `UTC` | — | no | Timezone for the output datetime (informational; UTC assumed unless offset in format) |

## How it works

Parses a datetime string in one format and re-writes it in another. Uses strftime/strptime format strings (e.g. %d/%m/%Y %H:%M:%S). Timezone names are noted but conversion uses UTC unless a numeric offset is embedded in the format.

## Implementation

The implementation is in `src/operations/translate_date_time_format.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Translate DateTime Format"
```

For file or binary input use `rxchef run "Translate DateTime Format" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Translate DateTime Format" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/translate_date_time_format.rs

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
