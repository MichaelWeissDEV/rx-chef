# BLAKE3

Hashes the input using BLAKE3 (UTF-8 encoded), with an optional key (also UTF-8), and outputs the result in hexadecimal format.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "BLAKE3"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Size (bytes) | `32` | Output size in bytes |
| 2 | Key | `<empty>` | Optional key for keyed hashing |

