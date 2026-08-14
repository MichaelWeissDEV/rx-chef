# AES Encrypt

## Overview

Advanced Encryption Standard (AES) is a U.S. Federal Information Processing Standard (FIPS). It was selected after a 5-year process where 15 competing designs were evaluated.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Encryption key (16, 24, or 32 bytes) |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | Initialization Vector (16 bytes, optional, defaults to null) |
| 3 | Mode | `Enum` | no | `CBC` | CBC, CFB, OFB, CTR, GCM, ECB | no | Cipher mode (CBC, CFB, OFB, CTR, GCM, ECB) |
| 4 | Input | `Enum` | no | `Raw` | Raw, Hex | no | Input encoding (Raw, Hex) |
| 5 | Output | `Enum` | no | `Hex` | Hex, Raw | no | Output encoding (Hex, Raw) |
| 6 | Additional Authenticated Data | `Bytes` | no | `<empty>` | — | no | AAD for GCM mode (optional) |

## How it works

Advanced Encryption Standard (AES) is a U.S. Federal Information Processing Standard (FIPS). It was selected after a 5-year process where 15 competing designs were evaluated.

## Implementation

The implementation is in `src/operations/aes_encrypt.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "AES Encrypt"
```

For file or binary input use `rxchef run "AES Encrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "AES Encrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/aes_encrypt.rs

Known-answer:
- none recorded

Differential:
- none recorded

Property:
- none recorded

Fuzz:
- none recorded

## Performance

Benchmark evidence:
- representative release benchmark

See [benchmark results](../performance/results.md) for measured environment and statistics.

## Limitations

No verified limitation metadata is currently recorded; this is not a claim of perfect upstream parity.

## References

- [Operation quality matrix](../reference/operation-matrix.md)
- [CLI run documentation](../cli/run.md)
