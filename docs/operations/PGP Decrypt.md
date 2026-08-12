# PGP Decrypt

Decrypts a PGP-encrypted message using the recipient's ASCII-armoured private key. Input: ASCII-armoured PGP message. Arguments: private key and optional passphrase.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "PGP Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Private key of recipient | `<empty>` | ASCII-armoured PGP private key |
| 2 | Private key passphrase | `<empty>` | Passphrase for the private key (leave blank if none) |

