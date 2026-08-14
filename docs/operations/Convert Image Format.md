# Convert Image Format

## Overview

Converts an image between different formats. Supported formats:<br><ul><li>Joint Photographic Experts Group (JPEG)</li><li>Portable Network Graphics (PNG)</li><li>Bitmap (BMP)</li><li>Tagged Image File Format (TIFF)</li></ul><br>Note: GIF files are supported for input, but cannot be outputted.

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
| 1 | Output Format | `String` | no | `JPEG` | — | no | The format to convert the image to |
| 2 | JPEG Quality | `Integer` | no | `80` | — | no | The quality of the JPEG output (1-100) |
| 3 | PNG Filter Type | `String` | no | `Auto` | — | no | The filter type to use for PNG output |
| 4 | PNG Deflate Level | `Integer` | no | `9` | — | no | The deflate level to use for PNG output (0-9) |

## How it works

Converts an image between different formats. Supported formats:<br><ul><li>Joint Photographic Experts Group (JPEG)</li><li>Portable Network Graphics (PNG)</li><li>Bitmap (BMP)</li><li>Tagged Image File Format (TIFF)</li></ul><br>Note: GIF files are supported for input, but cannot be outputted.

## Implementation

The implementation is in `src/operations/convert_image_format.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Convert Image Format"
```

For file or binary input use `rxchef run "Convert Image Format" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Convert Image Format" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/convert_image_format.rs

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
