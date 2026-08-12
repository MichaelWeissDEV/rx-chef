# Whirlpool

Whirlpool is a cryptographic hash function designed by Vincent Rijmen (co-creator of AES) and Paulo S. L. M. Barreto, who first described it in 2000. Whirlpool is the latest revision, released in 2003, fixing a flaw in the diffusion matrix.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Whirlpool"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Variant | `Whirlpool` | Hash variant (Whirlpool only) |
| 2 | Rounds | `10` | Number of rounds (1-10) |

