# Add Text To Image

## Overview

Adds text onto an image.<br><br>Text can be horizontally or vertically aligned, or the position can be manually specified.

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
| 1 | Text | `String` | no | `<empty>` | — | no | The text to add. |
| 2 | Horizontal align | `String` | no | `None` | — | no | None, Left, Center, Right |
| 3 | Vertical align | `String` | no | `None` | — | no | None, Top, Middle, Bottom |
| 4 | X position | `Integer` | no | `0` | — | no | Manual X position |
| 5 | Y position | `Integer` | no | `0` | — | no | Manual Y position |
| 6 | Size | `UnsignedInteger` | no | `32` | — | no | Font size |
| 7 | Red | `Integer` | no | `255` | — | no | Red component (0-255) |
| 8 | Green | `Integer` | no | `255` | — | no | Green component (0-255) |
| 9 | Blue | `Integer` | no | `255` | — | no | Blue component (0-255) |
| 10 | Alpha | `Integer` | no | `255` | — | no | Alpha component (0-255) |

## Implementation

The implementation is in `src/operations/add_text_to_image.rs` and declares `Bytes` input and `Bytes` output. The operation module owns conversion and domain-error rules; registry resolution, argument validation, input-requirement enforcement, tracing, and output validation are performed by `rxchef::execution`.

## Command-line use

This operation requires input. Supply literal UTF-8 with `--input`, exact bytes with `--input-file`, or pipe bytes on stdin.

```console
rxchef run "Add Text To Image" --input-file input.bin --output-file output.bin
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
- tests/tests/operations/add_text_to_image.rs

## Performance classification

Excluded from the committed representative benchmark set: No stable representative benchmark case is defined; operation remains Partial until performance evidence is reviewed.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [Operation arguments](../concepts/operation-arguments.md)
- [CLI run documentation](../cli/run.md)
