# SM4 Encrypt

## Overview

SM4 is a 128-bit block cipher, currently established as a national standard (GB/T 32907-2016) of China. Multiple block cipher modes are supported. When using CBC or ECB mode, the PKCS#7 padding scheme is used.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Encryption key (16 bytes, 128 bits) |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | Initialization Vector (16 bytes for CBC/CFB/OFB/CTR modes) |
| 3 | Mode | `Enum` | no | `CBC` | CBC, CFB, OFB, CTR, ECB, CBC/NoPadding, ECB/NoPadding | no | Cipher mode (CBC, CFB, OFB, CTR, ECB, CBC/NoPadding, ECB/NoPadding) |
| 4 | Input | `Enum` | no | `Raw` | Raw, Hex | no | Input encoding (Raw, Hex) |
| 5 | Output | `Enum` | no | `Hex` | Hex, Raw | no | Output encoding (Hex, Raw) |

## How it works

SM4 is a 128-bit block cipher, currently established as a national standard (GB/T 32907-2016) of China. Multiple block cipher modes are supported. When using CBC or ECB mode, the PKCS#7 padding scheme is used.

## Implementation

The implementation is in `src/operations/sm4_encrypt.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "SM4 Encrypt"
```

For file or binary input use `rxchef run "SM4 Encrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "SM4 Encrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/sm4_encrypt.rs

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
