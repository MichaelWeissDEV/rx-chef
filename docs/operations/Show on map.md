# Show on map

## Overview

Displays comma-separated coordinates on an OpenStreetMap slippy map. Decimal degrees (DD), degrees/decimal minutes (DDM), and degrees/minutes/seconds (DMS) with N/S/E/W suffixes are converted to decimal degrees. Map tiles require network access in the HTML viewer.

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
| 1 | Zoom Level | `Integer` | no | `13` | — | no | Zoom level of the map (0-20) |
| 2 | Input Format | `String` | no | `Auto` | — | no | Format of the input coordinates |
| 3 | Input Delimiter | `String` | no | `Auto` | — | no | Delimiter separating the coordinates |

## How it works

Displays comma-separated coordinates on an OpenStreetMap slippy map. Decimal degrees (DD), degrees/decimal minutes (DDM), and degrees/minutes/seconds (DMS) with N/S/E/W suffixes are converted to decimal degrees. Map tiles require network access in the HTML viewer.

## Implementation

The implementation is in `src/operations/show_on_map.rs` and declares `String` input and `HTML` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Show on map"
```

For file or binary input use `rxchef run "Show on map" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Show on map" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/show_on_map.rs

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
