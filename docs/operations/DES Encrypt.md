# DES Encrypt

DES is a previously dominant algorithm for encryption, and was published as an official U.S. Federal Information Processing Standard (FIPS). It is now considered to be insecure due to its small key size.<br><br><b>Key:</b> DES uses a key length of 8 bytes (64 bits).<br><br>You can generate a password-based key using one of the KDF operations.<br><br><b>IV:</b> The Initialization Vector should be 8 bytes long. If not entered, it will default to 8 null bytes.<br><br><b>Padding:</b> In CBC and ECB mode, PKCS#7 padding will be used.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "DES Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Encryption key (8 bytes) |
| 2 | IV | `<empty>` | Initialization Vector (8 bytes) |
| 3 | Mode | `CBC` | Cipher mode (CBC, CFB, OFB, CTR, ECB) |
| 4 | Input | `Raw` | Input encoding (Raw, Hex) |
| 5 | Output | `Hex` | Output encoding (Hex, Raw) |

