# RSA Verify

Verify a message against a signature and a public PEM encoded RSA key.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "RSA Verify"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | RSA Public Key (PEM) | `-----BEGIN RSA PUBLIC KEY-----` | RSA Public Key (PEM) |
| 2 | Message | `<empty>` | Message to verify |
| 3 | Message format | `Raw` | Format of the message |
| 4 | Message Digest Algorithm | `SHA-256` | Message Digest Algorithm |

