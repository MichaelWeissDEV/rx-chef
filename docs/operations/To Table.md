# To Table

## Overview

Data can be split on different characters and rendered as an HTML, ASCII or Markdown table with an optional header row.<br><br>Supports the CSV (Comma Separated Values) file format by default. Change the cell delimiter argument to <code>\t</code> to support TSV (Tab Separated Values) or <code>|</code> for PSV (Pipe Separated Values).<br><br>You can enter as many delimiters as you like. Each character will be treat as a separate possible delimiter.

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
| 1 | Cell delimiters | `String` | no | `,` | — | no | Characters used to separate cells |
| 2 | Row delimiters | `String` | no | `\\r\\n` | — | no | Characters used to separate rows |
| 3 | Make first row header | `Boolean` | no | `false` | — | no | Treat the first row as a header row |
| 4 | Format | `String` | no | `ASCII` | — | no | The output format |

## How it works

Data can be split on different characters and rendered as an HTML, ASCII or Markdown table with an optional header row.<br><br>Supports the CSV (Comma Separated Values) file format by default. Change the cell delimiter argument to <code>\t</code> to support TSV (Tab Separated Values) or <code>|</code> for PSV (Pipe Separated Values).<br><br>You can enter as many delimiters as you like. Each character will be treat as a separate possible delimiter.

## Implementation

The implementation is in `src/operations/to_table.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "To Table"
```

For file or binary input use `rxchef run "To Table" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "To Table" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/to_table.rs

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
