# RC2 Encrypt

## Overview

RC2 (also known as ARC2) is a symmetric-key block cipher designed by Ron Rivest in 1987. Supports CBC mode (8-byte IV) or ECB mode (empty IV). Uses PKCS#7 padding.

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
| 1 | Key | `Bytes` | yes | `<empty>` | — | yes | Encryption key as UTF-8 or hex (prefix 0x for hex) |
| 2 | IV | `Bytes` | no | `<empty>` | — | no | Initialization vector (8 bytes for CBC, empty for ECB) |
| 3 | Input | `String` | no | `Raw` | — | no | Input encoding: Raw or Hex |
| 4 | Output | `String` | no | `Hex` | — | no | Output encoding: Hex or Raw |

## How it works

RC2 (also known as ARC2) is a symmetric-key block cipher designed by Ron Rivest in 1987. Supports CBC mode (8-byte IV) or ECB mode (empty IV). Uses PKCS#7 padding.

## Implementation

The implementation is in `src/operations/rc2_encrypt.rs` and declares `Bytes` input and `Bytes` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "RC2 Encrypt"
```

For file or binary input use `rxchef run "RC2 Encrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "RC2 Encrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/rc2_encrypt.rs

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
