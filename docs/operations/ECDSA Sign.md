# ECDSA Sign

Sign a plaintext message with a PEM encoded EC key.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "ECDSA Sign"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | ECDSA Private Key (PEM) | `-----BEGIN EC PRIVATE KEY-----` | The PEM encoded ECDSA private key |
| 2 | Message Digest Algorithm | `SHA-256` | The hash algorithm to use |
| 3 | Output Format | `ASN.1 HEX` | The format of the output signature |

