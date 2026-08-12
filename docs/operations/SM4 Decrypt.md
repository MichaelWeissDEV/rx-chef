# SM4 Decrypt

SM4 is a 128-bit block cipher, currently established as a national standard (GB/T 32907-2016) of China. Multiple block cipher modes are supported. When using CBC or ECB mode, the PKCS#7 padding scheme is used.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "SM4 Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Decryption key (16 bytes, 128 bits) |
| 2 | IV | `<empty>` | Initialization Vector (16 bytes for CBC/CFB/OFB/CTR modes) |
| 3 | Mode | `CBC` | Cipher mode (CBC, CFB, OFB, CTR, ECB, CBC/NoPadding, ECB/NoPadding) |
| 4 | Input | `Hex` | Input encoding (Raw, Hex) |
| 5 | Output | `Raw` | Output encoding (Raw, Hex) |

