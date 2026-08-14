# Contain Image

## Overview

Scales an image to the specified width and height, maintaining the aspect ratio. The image may be letterboxed.

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
| 1 | Width | `UnsignedInteger` | no | `100` | — | no | The width of the contained image |
| 2 | Height | `UnsignedInteger` | no | `100` | — | no | The height of the contained image |
| 3 | Horizontal align | `String` | no | `Center` | — | no | The horizontal alignment of the image within the container |
| 4 | Vertical align | `String` | no | `Middle` | — | no | The vertical alignment of the image within the container |
| 5 | Resizing algorithm | `String` | no | `Bilinear` | — | no | The algorithm to use when resizing the image |
| 6 | Opaque background | `Boolean` | no | `true` | — | no | Whether to use an opaque black background instead of transparency |

## How it works

Scales an image to the specified width and height, maintaining the aspect ratio. The image may be letterboxed.

## Implementation

The implementation is in `src/operations/contain_image.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Contain Image"
```

For file or binary input use `rxchef run "Contain Image" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Contain Image" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/contain_image.rs

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
