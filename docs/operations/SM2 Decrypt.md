# SM2 Decrypt

Decrypts a message utilizing the SM2 standard. SM2 is a public-key cryptography standard used in China.

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "SM2 Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Private Key | `<empty>` | The private key in hex format (32 bytes) |
| 2 | Input Format | `C1C3C2` | The format of the input ciphertext (C1C3C2 or C1C2C3) |
| 3 | Curve | `sm2p256v1` | The elliptic curve to use (sm2p256v1) |

