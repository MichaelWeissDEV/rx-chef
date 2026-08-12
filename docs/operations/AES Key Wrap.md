# AES Key Wrap

A key wrapping algorithm defined in RFC3394, which is used to protect keys in untrusted storage or communications, using AES.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "AES Key Wrap"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key (KEK) | `<empty>` | Key-encryption key (16, 24, or 32 bytes) |
| 2 | IV | `a6a6a6a6a6a6a6a6` | Initialization Vector (8 bytes, defaults to a6a6a6a6a6a6a6a6) |
| 3 | Input | `Hex` | Input encoding (Raw, Hex) |
| 4 | Output | `Hex` | Output encoding (Raw, Hex) |

