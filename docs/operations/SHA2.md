# SHA2

## Overview

The SHA-2 (Secure Hash Algorithm 2) hash functions were designed by the NSA. SHA-2 includes significant changes from its predecessor, SHA-1. The SHA-2 family consists of hash functions with digests (hash values) that are 224, 256, 384 or 512 bits: SHA224, SHA256, SHA384, SHA512. The message digest algorithm for SHA256 variants consists, by default, of 64 rounds, and for SHA512 variants, it is, by default, 160.

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
| 1 | Size | `Enum` | no | `256` | 224, 256, 384, 512, 512/256, 512/224 | no | Output size (224, 256, 384, 512, 512/256, 512/224) |
| 2 | Rounds (SHA-256) | `UnsignedInteger` | no | `64` | — | no | Number of rounds for 256/224 (minimum 16) |
| 3 | Rounds (SHA-512) | `UnsignedInteger` | no | `160` | — | no | Number of rounds for 512/384/224/256 (minimum 32) |

## How it works

The SHA-2 (Secure Hash Algorithm 2) hash functions were designed by the NSA. SHA-2 includes significant changes from its predecessor, SHA-1. The SHA-2 family consists of hash functions with digests (hash values) that are 224, 256, 384 or 512 bits: SHA224, SHA256, SHA384, SHA512. The message digest algorithm for SHA256 variants consists, by default, of 64 rounds, and for SHA512 variants, it is, by default, 160.

## Implementation

The implementation is in `src/operations/sha2.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "SHA2"
```

For file or binary input use `rxchef run "SHA2" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "SHA2" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/sha2.rs

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
