# Series chart

## Overview

A time series graph is a line graph of repeated measurements taken over regular time intervals.

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
| 1 | Record delimiter | `String` | no | `\\n` | — | no | Character(s) that separate records |
| 2 | Field delimiter | `String` | no | `,` | — | no | Character(s) that separate fields |
| 3 | X label | `String` | no | `<empty>` | — | no | Label for the X axis |
| 4 | Point radius | `Integer` | no | `1` | — | no | Radius of points in the graph |
| 5 | Series colours | `String` | no | `mediumseagreen, dodgerblue, tomato` | — | no | Comma-separated list of colours for each series |

## How it works

A time series graph is a line graph of repeated measurements taken over regular time intervals.

## Implementation

The implementation is in `src/operations/series_chart.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Series chart"
```

For file or binary input use `rxchef run "Series chart" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Series chart" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/series_chart.rs

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
