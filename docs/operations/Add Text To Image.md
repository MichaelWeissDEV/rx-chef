# Add Text To Image

## Overview

Adds text onto an image.<br><br>Text can be horizontally or vertically aligned, or the position can be manually specified.

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
| 6 | Size | `Integer` | no | `32` | — | no | Font size |
| 7 | Red | `Integer` | no | `255` | — | no | Red component (0-255) |
| 8 | Green | `Integer` | no | `255` | — | no | Green component (0-255) |
| 9 | Blue | `Integer` | no | `255` | — | no | Blue component (0-255) |
| 10 | Alpha | `Integer` | no | `255` | — | no | Alpha component (0-255) |

## How it works

The shared execution engine validates the ordered arguments, passes the declared input representation to this operation, and validates the declared output contract. See the overview for the operation-specific format or algorithm.

## Implementation

Source module: `src/operations/add_text_to_image.rs`. Execution uses `rxchef::execute`; CLI, recipes, and the stdio server do not carry separate operation logic.

## Examples

```console
printf 'input' | rxchef run "Add Text To Image"
```

For file or binary input use `rxchef run "Add Text To Image" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Add Text To Image" to_base64
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
