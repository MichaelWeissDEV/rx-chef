# SNEFRU

SNEFRU is a cryptographic hash function invented by Ralph Merkle in 1990 while working at Xerox PARC. The function supports 128-bit and 256-bit output. The original design was shown to be insecure and was modified by increasing the number of iterations from two to eight.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "SNEFRU"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Size | `128` | Output size in bits (32-480, step 32) |
| 2 | Rounds | `8` | Number of rounds (2, 4, or 8) |

