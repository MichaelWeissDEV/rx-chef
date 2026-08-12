# GOST Decrypt

The GOST block cipher (Magma), defined in the standard GOST 28147-89 (RFC 5830), is a Soviet and Russian government standard symmetric key block cipher with a block size of 64 bits.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "GOST Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The decryption key. |
| 2 | IV | `<empty>` | The initialization vector. |
| 3 | Input type | `Hex` | Type of input data |
| 4 | Output type | `Raw` | Type of output data |
| 5 | Algorithm | `GOST R 34.12 (Magma, 2015)` | GOST version |
| 6 | sBox | `E-TEST` | S-Box to use (1989 only) |
| 7 | Block mode | `ECB` | Mode of operation |
| 8 | Key meshing mode | `NO` | Key meshing |
| 9 | Padding | `NO` | Padding scheme |

