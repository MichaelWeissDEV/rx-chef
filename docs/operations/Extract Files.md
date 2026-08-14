# Extract Files

## Overview

Carves PNG, JPEG, GIF, PDF, and ZIP signatures from binary input. Matching payloads are returned in a deterministic binary envelope with an ASCII file-type header before each payload.

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
| 1 | Images | `Boolean` | no | `true` | — | no | Extract images |
| 2 | Documents | `Boolean` | no | `true` | — | no | Extract documents |
| 3 | Archives | `Boolean` | no | `true` | — | no | Extract archives |
| 4 | Ignore failed extractions | `Boolean` | no | `true` | — | no | Ignore failed extractions |
| 5 | Minimum File Size | `UnsignedInteger` | no | `100` | — | no | Minimum file size to extract |

## How it works

Carves PNG, JPEG, GIF, PDF, and ZIP signatures from binary input. Matching payloads are returned in a deterministic binary envelope with an ASCII file-type header before each payload.

## Implementation

The implementation is in `src/operations/extract_files.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Extract Files"
```

For file or binary input use `rxchef run "Extract Files" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Extract Files" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/extract_files.rs

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
