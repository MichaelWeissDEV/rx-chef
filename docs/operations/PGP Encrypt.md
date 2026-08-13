# PGP Encrypt

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

Encrypts a message using the recipient's ASCII-armoured PGP public key. Input: plaintext message. Arguments: recipient's public key.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "PGP Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Public key of recipient | `<empty>` | ASCII-armoured PGP public key of the recipient |

