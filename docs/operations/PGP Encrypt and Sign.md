# PGP Encrypt and Sign

!!! warning "Optional backend unavailable"

    This operation is feature-gated and unavailable in the minimal documentation build. See the feature matrix for the required Cargo feature.

Encrypts a message to the recipient and signs it with the signer's private key. Input: cleartext to sign. Arguments: private key of signer, optional passphrase, public key of recipient.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "PGP Encrypt and Sign"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Private key of signer | `<empty>` | ASCII-armoured PGP private key of the signer |
| 2 | Private key passphrase | `<empty>` | Passphrase for the private key (leave blank if none) |
| 3 | Public key of recipient | `<empty>` | ASCII-armoured PGP public key of the recipient |

