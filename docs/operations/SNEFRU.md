# SNEFRU

Computes the standardized 256-bit, 8-round SNEFRU hash. SNEFRU was designed by Ralph Merkle in 1990; the original shorter-round design is retained in the argument schema for recipe compatibility but rejected because it is cryptographically broken.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "SNEFRU"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Size | `256` | Output size in bits (supported: 256) |
| 2 | Rounds | `8` | Number of rounds (supported: 8) |

