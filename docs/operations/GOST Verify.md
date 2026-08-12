# GOST Verify

Verify the signature of a plaintext message using one of the GOST block ciphers. Enter the signature in the MAC field.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "GOST Verify"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The decryption key. |
| 2 | IV | `<empty>` | The initialization vector. |
| 3 | MAC | `<empty>` | The signature/MAC to verify. |
| 4 | Input type | `Raw` | Type of input data |
| 5 | Algorithm | `GOST R 34.12 (Magma, 2015)` | GOST version |
| 6 | sBox | `E-TEST` | S-Box to use (1989 only) |

