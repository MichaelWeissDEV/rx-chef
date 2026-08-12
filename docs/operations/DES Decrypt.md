# DES Decrypt

DES is a previously dominant algorithm for encryption, and was published as an official U.S. Federal Information Processing Standard (FIPS). It is now considered to be insecure due to its small key size.<br><br><b>Key:</b> DES uses a key length of 8 bytes (64 bits).<br><br><b>IV:</b> The Initialization Vector should be 8 bytes long. If not entered, it will default to 8 null bytes.<br><br><b>Padding:</b> In CBC and ECB mode, PKCS#7 padding will be used as a default.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "DES Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Decryption key (8 bytes) |
| 2 | IV | `<empty>` | Initialization Vector (8 bytes, optional, defaults to null) |
| 3 | Mode | `CBC` | Cipher mode (CBC, CFB, OFB, CTR, ECB, CBC/NoPadding, ECB/NoPadding) |
| 4 | Input | `Hex` | Input encoding (Hex, Raw) |
| 5 | Output | `Raw` | Output encoding (Raw, Hex) |

