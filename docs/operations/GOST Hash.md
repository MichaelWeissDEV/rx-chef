# GOST Hash

The GOST hash function, defined in the standards GOST R 34.11-94 and GOST 34.311-95 is a 256-bit cryptographic hash function. It was initially defined in the Russian national standard GOST R 34.11-94 Information Technology  Cryptographic Information Security  Hash Function. The equivalent standard used by other member-states of the CIS is GOST 34.311-95.

This function must not be confused with a different Streebog hash function, which is defined in the new revision of the standard GOST R 34.11-2012.

The GOST hash function is based on the GOST block cipher.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "GOST Hash"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Algorithm | `GOST 28147 (1994)` | The GOST hash algorithm version to use. |
| 2 | Digest length | `256` | The length of the digest to produce (only for Streebog). |
| 3 | sBox | `E-TEST` | GOST94 parameter set: E-TEST/D-TEST (test) or CryptoPro/D-A |

