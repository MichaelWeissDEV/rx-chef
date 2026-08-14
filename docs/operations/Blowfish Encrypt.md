# Blowfish Encrypt

## Overview

Blowfish is a symmetric-key block cipher designed in 1993 by Bruce Schneier and included in a large number of cipher suites and encryption products. AES now receives more attention.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Encryption key (4-56 bytes) |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | Initialization Vector (8 bytes for non-ECB modes, optional) |
| 3 | Mode | `Enum` | no | `CBC` | CBC, CFB, OFB, CTR, ECB | no | Cipher mode (CBC, CFB, OFB, CTR, ECB) |
| 4 | Input | `Enum` | no | `Raw` | Raw, Hex | no | Input encoding (Raw, Hex) |
| 5 | Output | `Enum` | no | `Hex` | Hex, Raw | no | Output encoding (Hex, Raw) |

## How it works

Blowfish is a symmetric-key block cipher designed in 1993 by Bruce Schneier and included in a large number of cipher suites and encryption products. AES now receives more attention.

## Implementation

The implementation is in `src/operations/blowfish_encrypt.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Blowfish Encrypt"
```

For file or binary input use `rxchef run "Blowfish Encrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Blowfish Encrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/blowfish_encrypt.rs

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
