# PGP Encrypt and Sign

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

## Overview

Encrypts a message to the recipient and signs it with the signer's private key. Input: cleartext to sign. Arguments: private key of signer, optional passphrase, public key of recipient.

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
| 1 | Private key of signer | `Bytes` | yes | `<empty>` | — | yes | ASCII-armoured PGP private key of the signer |
| 2 | Private key passphrase | `Bytes` | yes | `<empty>` | — | yes | Passphrase for the private key (leave blank if none) |
| 3 | Public key of recipient | `String` | no | `<empty>` | — | no | ASCII-armoured PGP public key of the recipient |

## How it works

Encrypts a message to the recipient and signs it with the signer's private key. Input: cleartext to sign. Arguments: private key of signer, optional passphrase, public key of recipient.

## Implementation

The implementation is in `src/operations/pgp_encrypt_and_sign.rs` and declares `String` input and `String` output. Its operation module owns the conversion and error rules; every public frontend invokes it through `rxchef::execution`.

## Examples

```console
printf 'input' | rxchef run "PGP Encrypt and Sign"
```

For file or binary input use `rxchef run "PGP Encrypt and Sign" --input-file INPUT --output-file OUTPUT`.

## Pipeline usage

```console
printf 'input' | rxchef pipe "PGP Encrypt and Sign" to_base64
```

## Error conditions

Invalid input representations, invalid argument values, unavailable feature backends, and operation-specific processing failures return an error and a non-zero CLI status. Exact limitations are listed below when known.

## CyberChef compatibility

Parity status: `Unknown`. `Unknown` means compatibility has not been independently verified and must not be read as an exact-match claim.

## Security considerations

Side effects: `[]`. Treat parser inputs as untrusted and use execution limits for large data. Sensitive arguments are redacted by metadata-aware History output.

## Testing

Correctness:
- tests/tests/operations/pgp_encrypt_and_sign.rs

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
