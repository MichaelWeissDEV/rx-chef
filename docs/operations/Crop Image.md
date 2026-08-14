# Crop Image

## Overview

Crops an image to the specified region, or automatically crops edges.<br><br><b><u>Autocrop</u></b><br>Automatically crops same-colour borders from the image.<br><br><u>Autocrop tolerance</u><br>A percentage value for the tolerance of colour difference between pixels.<br><br><u>Only autocrop frames</u><br>Only crop real frames (all sides must have the same border)<br><br><u>Symmetric autocrop</u><br>Force autocrop to be symmetric (top/bottom and left/right are cropped by the same amount)<br><br><u>Autocrop keep border</u><br>The number of pixels of border to leave around the image.

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

## Implementation

The implementation is in `src/operations/crop_image.rs` and declares `Bytes` input and `Bytes` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Crop Image" --input-file input.bin --output-file output.bin
```

Arguments may be supplied positionally in the table order or by name with repeatable `--arg NAME=VALUE`. Omitted optional arguments use the documented defaults.

## Pipeline use

Place the operation anywhere a `Bytes` value is valid. Its `Bytes` result becomes the next step's input. Compact syntax uses the operation name followed by comma-separated arguments; JSON/YAML recipes use an `op` field and an `args` array.

## Error conditions

Schema violations are rejected before the operation runs. Malformed input, unsupported parameter combinations, unavailable optional backends, and domain processing failures produce structured errors and a non-zero CLI status; partial output is never reported as success.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` records an unassessed compatibility claim; it does not imply equality or incompatibility.

## Security considerations

Declared side effects: `[]`. Treat parser inputs as untrusted and apply execution limits to large data. Arguments marked sensitive in the schema are redacted from metadata-aware History displays.

## Testing evidence

Correctness tests:
- tests/tests/operations/crop_image.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
