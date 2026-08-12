# SM2 Encrypt

Encrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "SM2 Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Public Key X | `<empty>` | Public key component X in hex format (32 bytes) |
| 2 | Public Key Y | `<empty>` | Public key component Y in hex format (32 bytes) |
| 3 | Output Format | `C1C3C2` | The format of the output ciphertext (C1C3C2 or C1C2C3) |
| 4 | Curve | `sm2p256v1` | The elliptic curve to use (sm2p256v1) |

