# MD6

The MD6 (Message-Digest 6) algorithm is a cryptographic hash function. It uses a Merkle tree-like structure to allow for immense parallel computation of hashes for very long inputs.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "MD6"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Size | `256` | Hash size in bits (0-512) |
| 2 | Levels | `64` | Number of levels in the Merkle tree |
| 3 | Key | `<empty>` | Optional key |

