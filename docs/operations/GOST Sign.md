# GOST Sign

Sign a plaintext message (calculate MAC) using one of the GOST block ciphers.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "GOST Sign"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The encryption key. |
| 2 | IV | `<empty>` | The initialization vector. |
| 3 | Input type | `Raw` | Input encoding (Raw, Hex) |
| 4 | Output type | `Hex` | Output encoding (Hex, Raw) |
| 5 | Algorithm | `GOST 28147 (1989)` | The GOST algorithm to use. |
| 6 | sBox | `E-TEST` | The sBox to use (only for GOST 28147 (1989)). |
| 7 | MAC length | `32` | The length of the MAC in bits. |

