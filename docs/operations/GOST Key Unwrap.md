# GOST Key Unwrap

A decryptor for keys wrapped using one of the GOST block ciphers.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "GOST Key Unwrap"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The decryption key. |
| 2 | User Key Material | `<empty>` | UKM |
| 3 | Input type | `Hex` | Type of input data |
| 4 | Output type | `Raw` | Type of output data |
| 5 | Algorithm | `GOST R 34.12 (Magma, 2015)` | GOST version |
| 6 | sBox | `E-TEST` | S-Box to use (1989 only) |
| 7 | Key wrapping | `NO` | Key wrapping mode |

