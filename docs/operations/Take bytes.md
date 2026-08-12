# Take bytes

Takes a slice of the specified number of bytes from the data. Negative values are allowed.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Take bytes"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Start | `0` | Starting byte position (negative counts from end) |
| 2 | Length | `5` | Number of bytes to take (negative reverses direction) |
| 3 | Apply to each line | `false` | If true, apply operation to each line separately |

