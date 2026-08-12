# PGP Decrypt and Verify

Decrypts and verifies a PGP signed+encrypted message. Input: ASCII-armoured encrypted PGP message. Arguments: public key of signer, private key of recipient, optional passphrase.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "PGP Decrypt and Verify"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Public key of signer | `<empty>` | ASCII-armoured PGP public key of the signer |
| 2 | Private key of recipient | `<empty>` | ASCII-armoured PGP private key of the recipient |
| 3 | Private key password | `<empty>` | Passphrase for the private key (leave blank if none) |

