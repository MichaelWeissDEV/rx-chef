# Drop bytes

Cuts a slice of the specified number of bytes out of the data. Negative values are allowed.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Drop bytes"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Start | `0` | Starting byte position (can be negative) |
| 2 | Length | `5` | Number of bytes to drop (can be negative) |
| 3 | Apply to each line | `false` | Apply drop to each line separately |

