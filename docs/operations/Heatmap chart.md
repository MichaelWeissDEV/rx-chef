# Heatmap chart

## Overview

A heatmap is a graphical representation of data where the individual values contained in a matrix are represented as colors.

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
| 1 | Record delimiter | `String` | no | `Line feed` | — | no | The record delimiter |
| 2 | Field delimiter | `String` | no | `Comma` | — | no | The field delimiter |
| 3 | Number of vertical bins | `Integer` | no | `25` | — | no | Number of vertical bins |
| 4 | Number of horizontal bins | `Integer` | no | `25` | — | no | Number of horizontal bins |
| 5 | Use column headers as labels | `Boolean` | no | `true` | — | no | Use column headers as labels |
| 6 | X label | `String` | no | `<empty>` | — | no | X label |
| 7 | Y label | `String` | no | `<empty>` | — | no | Y label |
| 8 | Draw bin edges | `Boolean` | no | `false` | — | no | Draw bin edges |
| 9 | Min colour value | `String` | no | `white` | — | no | Min colour value |
| 10 | Max colour value | `String` | no | `black` | — | no | Max colour value |

## How it works

A heatmap is a graphical representation of data where the individual values contained in a matrix are represented as colors.

## Implementation

The implementation is in `src/operations/heatmap_chart.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Heatmap chart"
```

For file or binary input use `rxchef run "Heatmap chart" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Heatmap chart" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/heatmap_chart.rs

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
