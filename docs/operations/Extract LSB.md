# Extract LSB

## Overview

Extracts the Least Significant Bit data from each pixel in an image. This is a common way to hide data in Steganography.

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

Declared input type: `Bytes`.

## Output

Declared output type: `Bytes`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Colour Pattern #1 | `String` | no | `R` | — | no | Colour to extract from |
| 2 | Colour Pattern #2 | `String` | no | `<empty>` | — | no | Colour to extract from |
| 3 | Colour Pattern #3 | `String` | no | `<empty>` | — | no | Colour to extract from |
| 4 | Colour Pattern #4 | `String` | no | `<empty>` | — | no | Colour to extract from |
| 5 | Pixel Order | `String` | no | `Row` | — | no | Order to process pixels |
| 6 | Bit | `Integer` | no | `0` | — | no | Bit to extract (0-7) |

## How it works

Extracts the Least Significant Bit data from each pixel in an image. This is a common way to hide data in Steganography.

## Implementation

The implementation is in `src/operations/extract_lsb.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Extract LSB"
```

For file or binary input use `rxchef run "Extract LSB" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Extract LSB" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/extract_lsb.rs

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
