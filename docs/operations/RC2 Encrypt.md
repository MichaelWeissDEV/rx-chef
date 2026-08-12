# RC2 Encrypt

RC2 (also known as ARC2) is a symmetric-key block cipher designed by Ron Rivest in 1987. Supports CBC mode (8-byte IV) or ECB mode (empty IV). Uses PKCS#7 padding.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "RC2 Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Encryption key as UTF-8 or hex (prefix 0x for hex) |
| 2 | IV | `<empty>` | Initialization vector (8 bytes for CBC, empty for ECB) |
| 3 | Input | `Raw` | Input encoding: Raw or Hex |
| 4 | Output | `Hex` | Output encoding: Hex or Raw |

