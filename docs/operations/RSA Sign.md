# RSA Sign

Sign a plaintext message with a PEM encoded RSA key.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "RSA Sign"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | RSA Private Key (PEM) | `-----BEGIN RSA PRIVATE KEY-----` | RSA Private Key (PEM) |
| 2 | Key Password | `<empty>` | Password for the private key (if encrypted) |
| 3 | Message Digest Algorithm | `SHA-256` | Message Digest Algorithm |

