# Scan for Embedded Files

## Overview

Scans the data for potential embedded files by looking for magic bytes at all offsets. This operation is prone to false positives.

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

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Images | `Boolean` | no | `true` | — | no | Scan for image files |
| 2 | Video | `Boolean` | no | `true` | — | no | Scan for video files |
| 3 | Audio | `Boolean` | no | `true` | — | no | Scan for audio files |
| 4 | Documents | `Boolean` | no | `true` | — | no | Scan for document files |
| 5 | Applications | `Boolean` | no | `true` | — | no | Scan for application files |
| 6 | Archives | `Boolean` | no | `true` | — | no | Scan for archive files |

## How it works

Scans the data for potential embedded files by looking for magic bytes at all offsets. This operation is prone to false positives.

## Implementation

The implementation is in `src/operations/scan_for_embedded_files.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Scan for Embedded Files"
```

For file or binary input use `rxchef run "Scan for Embedded Files" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Scan for Embedded Files" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/scan_for_embedded_files.rs

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
