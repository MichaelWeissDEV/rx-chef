# GOST Decrypt

## Overview

The GOST block cipher (Magma), defined in the standard GOST 28147-89 (RFC 5830), is a Soviet and Russian government standard symmetric key block cipher with a block size of 64 bits.

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

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | The decryption key. |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | The initialization vector. |
| 3 | Input type | `String` | no | `Hex` | — | no | Type of input data |
| 4 | Output type | `String` | no | `Raw` | — | no | Type of output data |
| 5 | Algorithm | `String` | no | `GOST R 34.12 (Magma, 2015)` | — | no | GOST version |
| 6 | sBox | `String` | no | `E-TEST` | — | no | S-Box to use (1989 only) |
| 7 | Block mode | `String` | no | `ECB` | — | no | Mode of operation |
| 8 | Key meshing mode | `String` | no | `NO` | — | no | Key meshing |
| 9 | Padding | `String` | no | `NO` | — | no | Padding scheme |

## How it works

The GOST block cipher (Magma), defined in the standard GOST 28147-89 (RFC 5830), is a Soviet and Russian government standard symmetric key block cipher with a block size of 64 bits.

## Implementation

The implementation is in `src/operations/gost_decrypt.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "GOST Decrypt"
```

For file or binary input use `rxchef run "GOST Decrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "GOST Decrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/gost_decrypt.rs

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
