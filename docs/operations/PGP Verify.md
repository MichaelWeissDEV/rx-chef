# PGP Verify

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

Verifies a PGP clearsigned or signed+encrypted message using the signer's public key. Input: ASCII-armoured signed PGP message. Arguments: public key of the signer.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "PGP Verify"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Public key of signer | `<empty>` | ASCII-armoured PGP public key of the signer |

