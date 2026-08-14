# SM2 Decrypt

## Overview

Decrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China.

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

Declared output type: `Bytes`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Private Key | `Bytes` | yes | `<empty>` | — | yes | The private key in hex format (32 bytes) |
| 2 | Input Format | `String` | no | `C1C3C2` | — | no | The format of the input ciphertext (C1C3C2 or C1C2C3) |
| 3 | Curve | `String` | no | `sm2p256v1` | — | no | The elliptic curve to use (sm2p256v1) |

## How it works

Decrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China.

## Implementation

The implementation is in `src/operations/sm2_decrypt.rs` and declares `String` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "SM2 Decrypt"
```

For file or binary input use `rxchef run "SM2 Decrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "SM2 Decrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/sm2_decrypt.rs

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
