# XSalsa20

## Overview

XSalsa is an extended-nonce Salsa stream cipher designed by Daniel J. Bernstein. It uses a 32-byte key, a 24-byte nonce, and a 64-bit block counter. The standard 20-round cipher and reduced-round XSalsa12 and XSalsa8 variants are supported.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Key to use for encryption/decryption |
| 2 | Nonce | `Bytes` | no | `<empty>` | — | no | Nonce to use |
| 3 | Counter | `UnsignedInteger` | no | `0` | — | no | Starting counter value |
| 4 | Rounds | `UnsignedInteger` | no | `20` | — | no | Number of rounds |
| 5 | Input | `String` | no | `Raw` | — | no | Input format |
| 6 | Output | `String` | no | `Raw` | — | no | Output format |

## How it works

XSalsa is an extended-nonce Salsa stream cipher designed by Daniel J. Bernstein. It uses a 32-byte key, a 24-byte nonce, and a 64-bit block counter. The standard 20-round cipher and reduced-round XSalsa12 and XSalsa8 variants are supported.

## Implementation

The implementation is in `src/operations/x_salsa20.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "XSalsa20"
```

For file or binary input use `rxchef run "XSalsa20" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "XSalsa20" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/x_salsa20.rs

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
