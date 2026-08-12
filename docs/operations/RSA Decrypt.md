# RSA Decrypt

Decrypt an RSA encrypted message with a PEM encoded private key.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "RSA Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | RSA Private Key (PEM) | `-----BEGIN RSA PRIVATE KEY-----` | RSA Private Key (PEM) |
| 2 | Key Password | `<empty>` | Key Password |
| 3 | Encryption Scheme | `RSA-OAEP` | Encryption Scheme |
| 4 | Message Digest Algorithm | `SHA-256` | Message Digest Algorithm (for OAEP) |

