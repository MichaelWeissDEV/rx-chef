# Zip

## Overview

Compresses data using the PKZIP algorithm with the given filename.<br><br>No support for multiple files at this time.

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
| 1 | Filename | `Path` | no | `file.txt` | — | no | Name of the file to be zipped |
| 2 | Comment | `String` | no | `<empty>` | — | no | Comment to add to the zip file |
| 3 | Password | `Bytes` | no | `<empty>` | — | yes | Password to protect the zip file (ZipCrypto) |
| 4 | Compression method | `String` | no | `Deflate` | — | no | Method to use for compression |
| 5 | Operating system | `String` | no | `Unix` | — | no | OS to set in the zip metadata |
| 6 | Compression type | `Enum` | no | `Dynamic` | Fixed, Dynamic, None | no | Compression level/type (Fixed, Dynamic, None) |

## How it works

Compresses data using the PKZIP algorithm with the given filename.<br><br>No support for multiple files at this time.

## Implementation

The implementation is in `src/operations/zip.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Zip"
```

For file or binary input use `rxchef run "Zip" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Zip" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/zip.rs

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
