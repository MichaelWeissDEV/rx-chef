# CSV to JSON

## Overview

Converts a CSV file to JSON format. The first row is used as the header for 'Array of dictionaries' format. Supports quoted fields with embedded delimiters and escaped double-quotes.

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

Declared output type: `JSON`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Cell delimiter | `String` | no | `,` | — | no | Character used to separate fields |
| 2 | Row delimiter | `String` | no | ` ` | — | no | Character(s) used to separate rows |
| 3 | Format | `String` | no | `Array of dictionaries` | — | no | Output format: 'Array of dictionaries' or 'Array of arrays' |

## How it works

Converts a CSV file to JSON format. The first row is used as the header for 'Array of dictionaries' format. Supports quoted fields with embedded delimiters and escaped double-quotes.

## Implementation

The implementation is in `src/operations/csv_to_json.rs` and declares `String` input and `JSON` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "CSV to JSON"
```

For file or binary input use `rxchef run "CSV to JSON" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "CSV to JSON" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/csv_to_json.rs

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
