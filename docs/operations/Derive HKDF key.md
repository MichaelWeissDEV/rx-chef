# Derive HKDF key

## Overview

A simple Hashed Message Authenticaton Code (HMAC)-based key derivation function (HKDF), defined in RFC5869.

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
| 1 | Salt | `Bytes` | no | `<empty>` | — | no | The salt to use |
| 2 | Info | `String` | no | `<empty>` | — | no | The info to use |
| 3 | Hashing function | `Enum` | no | `SHA256` | SHA1, SHA256, SHA384, SHA512 | no | The hashing function to use (SHA1, SHA256, SHA384, SHA512) |
| 4 | Extract mode | `Enum` | no | `with salt` | with salt, no salt, skip | no | The extract mode (with salt, no salt, skip) |
| 5 | L (number of output octets) | `Integer` | no | `16` | — | no | The number of output octets |

## How it works

A simple Hashed Message Authenticaton Code (HMAC)-based key derivation function (HKDF), defined in RFC5869.

## Implementation

The implementation is in `src/operations/derive_hkdf_key.rs` and declares `Bytes` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "Derive HKDF key"
```

For file or binary input use `rxchef run "Derive HKDF key" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "Derive HKDF key" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/derive_hkdf_key.rs

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
