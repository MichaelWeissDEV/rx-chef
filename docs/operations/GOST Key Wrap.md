# GOST Key Wrap

A key wrapping algorithm for protecting keys in untrusted storage using one of the GOST block ciphers.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "GOST Key Wrap"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The Key Encryption Key (KEK). |
| 2 | User Key Material | `<empty>` | User Key Material (UKM). |
| 3 | Input type | `Raw` | Input encoding (Raw, Hex) |
| 4 | Output type | `Hex` | Output encoding (Hex, Raw) |
| 5 | Algorithm | `GOST 28147 (1989)` | The GOST algorithm to use. |
| 6 | sBox | `E-TEST` | The sBox to use (only for GOST 28147 (1989)). |
| 7 | Key wrapping | `NO` | The key wrapping mode. |

