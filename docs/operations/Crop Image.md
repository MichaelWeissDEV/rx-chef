# Crop Image

## Overview

Crops an image to the specified region, or automatically crops edges.<br><br><b><u>Autocrop</u></b><br>Automatically crops same-colour borders from the image.<br><br><u>Autocrop tolerance</u><br>A percentage value for the tolerance of colour difference between pixels.<br><br><u>Only autocrop frames</u><br>Only crop real frames (all sides must have the same border)<br><br><u>Symmetric autocrop</u><br>Force autocrop to be symmetric (top/bottom and left/right are cropped by the same amount)<br><br><u>Autocrop keep border</u><br>The number of pixels of border to leave around the image.

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
| 1 | X Position | `Integer` | no | `0` | — | no | The x-coordinate of the top-left corner of the crop area |
| 2 | Y Position | `Integer` | no | `0` | — | no | The y-coordinate of the top-left corner of the crop area |
| 3 | Width | `UnsignedInteger` | no | `10` | — | no | The width of the crop area |
| 4 | Height | `UnsignedInteger` | no | `10` | — | no | The height of the crop area |
| 5 | Autocrop | `Boolean` | no | `false` | — | no | Whether to automatically crop borders |
| 6 | Autocrop tolerance (%) | `Integer` | no | `2` | — | no | The tolerance for color difference when autocropping |
| 7 | Only autocrop frames | `Boolean` | no | `true` | — | no | Only crop if all sides have the same border |
| 8 | Symmetric autocrop | `Boolean` | no | `false` | — | no | Force autocrop to be symmetric |
| 9 | Autocrop keep border (px) | `Integer` | no | `0` | — | no | The number of pixels of border to leave |

## How it works

Crops an image to the specified region, or automatically crops edges.<br><br><b><u>Autocrop</u></b><br>Automatically crops same-colour borders from the image.<br><br><u>Autocrop tolerance</u><br>A percentage value for the tolerance of colour difference between pixels.<br><br><u>Only autocrop frames</u><br>Only crop real frames (all sides must have the same border)<br><br><u>Symmetric autocrop</u><br>Force autocrop to be symmetric (top/bottom and left/right are cropped by the same amount)<br><br><u>Autocrop keep border</u><br>The number of pixels of border to leave around the image.

## Implementation

The implementation is in `src/operations/crop_image.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Crop Image"
```

For file or binary input use `rxchef run "Crop Image" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Crop Image" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/crop_image.rs

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
