# Bzip2 Compress

Bzip2 is a compression library developed by Julian Seward (of GHC fame) that uses the Burrows-Wheeler algorithm. It only supports compressing single files and its compression is slow, however is more effective than Deflate (.gz & .zip).

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Bzip2 Compress"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Block size (100s of kb) | `9` | Block size for compression (1-9) |
| 2 | Work factor | `30` | Effort spent on difficult data (0-250, 30 is default) |

