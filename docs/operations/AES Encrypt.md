# AES Encrypt

Advanced Encryption Standard (AES) is a U.S. Federal Information Processing Standard (FIPS). It was selected after a 5-year process where 15 competing designs were evaluated.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "AES Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Encryption key (16, 24, or 32 bytes) |
| 2 | IV | `<empty>` | Initialization Vector (16 bytes, optional, defaults to null) |
| 3 | Mode | `CBC` | Cipher mode (CBC, CFB, OFB, CTR, GCM, ECB) |
| 4 | Input | `Raw` | Input encoding (Raw, Hex) |
| 5 | Output | `Hex` | Output encoding (Hex, Raw) |
| 6 | Additional Authenticated Data | `<empty>` | AAD for GCM mode (optional) |

