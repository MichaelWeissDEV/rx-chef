# AES Key Unwrap

## Overview

Decryptor for a key wrapping algorithm defined in RFC3394, which is used to protect keys in untrusted storage or communications, using AES.

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
| 1 | Key (KEK) | `String` | no | `<empty>` | — | no | Key-encryption key (16, 24, or 32 bytes) |
| 2 | IV | `Bytes` | no | `a6a6a6a6a6a6a6a6` | — | no | Initialization Vector (8 bytes, defaults to a6a6a6a6a6a6a6a6) |
| 3 | Input | `Enum` | no | `Hex` | Raw, Hex | no | Input encoding (Raw, Hex) |
| 4 | Output | `Enum` | no | `Hex` | Raw, Hex | no | Output encoding (Raw, Hex) |

## How it works

Decryptor for a key wrapping algorithm defined in RFC3394, which is used to protect keys in untrusted storage or communications, using AES.

## Implementation

The implementation is in `src/operations/aes_key_unwrap.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "AES Key Unwrap"
```

For file or binary input use `rxchef run "AES Key Unwrap" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "AES Key Unwrap" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/aes_key_unwrap.rs

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
