# PGP Decrypt

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Decrypts a PGP-encrypted message using the recipient's ASCII-armoured private key. Input: ASCII-armoured PGP message. Arguments: private key and optional passphrase.

## Status

| Field | Value |
|---|---|
| Implementation | `Partial` |
| Parity | `Unknown` |
| Availability | FeatureDisabled |
| Features | pgp |
| Side effects | `[]` |
| Deterministic | true |

## Input

Declared input type: `String`.

## Output

Declared output type: `String`. Redirect stdout or use `--output-file` for exact binary bytes.

## Arguments

| # | Argument | Type | Required | Default | Allowed | Sensitive | Description |
|---:|---|---|:---:|---|---|:---:|---|
| 1 | Private key of recipient | `Bytes` | yes | `<empty>` | — | yes | ASCII-armoured PGP private key |
| 2 | Private key passphrase | `Bytes` | yes | `<empty>` | — | yes | Passphrase for the private key (leave blank if none) |

## How it works

Decrypts a PGP-encrypted message using the recipient's ASCII-armoured private key. Input: ASCII-armoured PGP message. Arguments: private key and optional passphrase.

## Implementation

The implementation is in `src/operations/pgp_decrypt.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "PGP Decrypt"
```

For file or binary input use `rxchef run "PGP Decrypt" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "PGP Decrypt" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/pgp_decrypt.rs

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
