# Hex Density chart

## Overview

Hex density charts are used in a similar way to scatter charts, however rather than rendering tens of thousands of points, it groups the points into a few hundred hexagons to show the distribution.

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
| 1 | Record delimiter | `String` | no | `\\n` | — | no | Delimiter between records |
| 2 | Field delimiter | `String` | no | `,` | — | no | Delimiter between fields |
| 3 | Pack radius | `Integer` | no | `25` | — | no | Radius of the hexagons |
| 4 | Draw radius | `Integer` | no | `15` | — | no | Radius of the hexagons to draw |
| 5 | Use column headers as labels | `Boolean` | no | `true` | — | no | Whether to use the first row as headers |
| 6 | X label | `String` | no | `<empty>` | — | no | Label for the X axis |
| 7 | Y label | `String` | no | `<empty>` | — | no | Label for the Y axis |
| 8 | Draw hexagon edges | `Boolean` | no | `false` | — | no | Whether to draw edges around hexagons |
| 9 | Min colour value | `String` | no | `#ffffff` | — | no | Colour for low density |
| 10 | Max colour value | `String` | no | `#000000` | — | no | Colour for high density |
| 11 | Draw empty hexagons within data boundaries | `Boolean` | no | `false` | — | no | Whether to draw empty hexagons |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/hex_density_chart.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Hex Density chart"
```

For file or binary input use `rxchef run "Hex Density chart" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Hex Density chart" to_base64
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
