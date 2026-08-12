# ECDSA Verify

Verify a message against a signature and a public PEM encoded EC key.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "ECDSA Verify"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input Format | `Auto` | The format of the input signature |
| 2 | Message Digest Algorithm | `SHA-256` | The hash algorithm to use |
| 3 | ECDSA Public Key (PEM) | `-----BEGIN PUBLIC KEY-----` | The PEM encoded ECDSA public key |
| 4 | Message | `<empty>` | The message to verify |
| 5 | Message format | `Raw` | The format of the message |

