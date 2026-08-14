# SM2 Encrypt

## Overview

Encrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China.

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
| 1 | Public Key X | `HexBytes` | no | `<empty>` | — | no | Public key component X in hex format (32 bytes) |
| 2 | Public Key Y | `HexBytes` | no | `<empty>` | — | no | Public key component Y in hex format (32 bytes) |
| 3 | Output Format | `String` | no | `C1C3C2` | — | no | The format of the output ciphertext (C1C3C2 or C1C2C3) |
| 4 | Curve | `String` | no | `sm2p256v1` | — | no | The elliptic curve to use (sm2p256v1) |

## How it works

Encrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China.

## Implementation

The implementation is in `src/operations/sm2_encrypt.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "SM2 Encrypt"
```

For file or binary input use `rxchef run "SM2 Encrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "SM2 Encrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/sm2_encrypt.rs

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
