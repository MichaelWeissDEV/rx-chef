# Heatmap chart

## Overview

A heatmap is a graphical representation of data where the individual values contained in a matrix are represented as colors.

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

## Implementation

The implementation is in `src/operations/heatmap_chart.rs` and declares `String` input and `HTML` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Heatmap chart" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `String` value is valid. Its `HTML` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/heatmap_chart.rs

Known-answer tests:
- tests/tests/operations/heatmap_chart.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
