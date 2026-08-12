# BLAKE2s

Performs BLAKE2s hashing on the input. BLAKE2s is a flavour of the BLAKE cryptographic hash function that is optimized for 8- to 32-bit platforms and produces digests of any size between 1 and 32 bytes. Supports the use of an optional key.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "BLAKE2s"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Size | `256` | Output size in bits (256, 160, 128) |
| 2 | Output Encoding | `Hex` | Output encoding (Hex, Base64, Raw) |
| 3 | Key | `<empty>` | Optional key for keyed hashing |

