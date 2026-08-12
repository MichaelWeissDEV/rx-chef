# BLAKE2b

Performs BLAKE2b hashing on the input. BLAKE2b is a flavour of the BLAKE cryptographic hash function that is optimized for 64-bit platforms and produces digests of any size between 1 and 64 bytes. Supports the use of an optional key.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "BLAKE2b"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Size | `512` | Output size in bits (512, 384, 256, 160, 128) |
| 2 | Output Encoding | `Hex` | Output encoding (Hex, Base64, Raw) |
| 3 | Key | `<empty>` | Optional key for keyed hashing |

