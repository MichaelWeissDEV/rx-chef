# RSA Encrypt

Encrypt a message with a PEM encoded RSA public key.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "RSA Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | RSA Public Key (PEM) | `-----BEGIN RSA PUBLIC KEY-----` | RSA Public Key (PEM) |
| 2 | Encryption Scheme | `RSA-OAEP` | Encryption Scheme |
| 3 | Message Digest Algorithm | `SHA-256` | Message Digest Algorithm (for OAEP) |

