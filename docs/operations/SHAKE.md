# SHAKE

SHAKE is an Extendable Output Function (XOF) of the SHA-3 hash algorithm, part of the Keccak family, allowing for variable output length/size.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "SHAKE"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Capacity | `256` | Capacity (256 or 128) |
| 2 | Size | `512` | Output size in bytes (minimum 1) |

