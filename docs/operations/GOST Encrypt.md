# GOST Encrypt

The GOST block cipher (Magma), defined in the standard GOST 28147-89 (RFC 5830), is a Soviet and Russian government standard symmetric key block cipher with a block size of 64 bits. The original standard, published in 1989, did not give the cipher any name, but the most recent revision of the standard, GOST R 34.12-2015 (RFC 7801, RFC 8891), specifies that it may be referred to as Magma. The GOST hash function is based on this cipher. The new standard also specifies a new 128-bit block cipher called Kuznyechik.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "GOST Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The encryption key. |
| 2 | IV | `<empty>` | The initialization vector. |
| 3 | Input type | `Raw` | Input encoding (Raw, Hex) |
| 4 | Output type | `Hex` | Output encoding (Hex, Raw) |
| 5 | Algorithm | `GOST 28147 (1989)` | The GOST algorithm to use. |
| 6 | sBox | `E-TEST` | The sBox to use (only for GOST 28147 (1989)). |
| 7 | Block mode | `ECB` | The block cipher mode to use. |
| 8 | Key meshing mode | `NO` | The key meshing mode to use. |
| 9 | Padding | `PKCS5` | The padding scheme to use. |

