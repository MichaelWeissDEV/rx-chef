# Blowfish Encrypt

Blowfish is a symmetric-key block cipher designed in 1993 by Bruce Schneier and included in a large number of cipher suites and encryption products. AES now receives more attention.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Blowfish Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Encryption key (4-56 bytes) |
| 2 | IV | `<empty>` | Initialization Vector (8 bytes for non-ECB modes, optional) |
| 3 | Mode | `CBC` | Cipher mode (CBC, CFB, OFB, CTR, ECB) |
| 4 | Input | `Raw` | Input encoding (Raw, Hex) |
| 5 | Output | `Hex` | Output encoding (Hex, Raw) |

