# RC2 Decrypt

RC2 (also known as ARC2) is a symmetric-key block cipher designed by Ron Rivest in 1987. Supports CBC mode (8-byte IV) or ECB mode (empty IV). Uses PKCS#7 padding.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "RC2 Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Decryption key as UTF-8 or hex (prefix 0x for hex) |
| 2 | IV | `<empty>` | Initialization vector (8 bytes for CBC, empty for ECB) |
| 3 | Input | `Hex` | Input encoding: Hex or Raw |
| 4 | Output | `Raw` | Output encoding: Raw or Hex |

