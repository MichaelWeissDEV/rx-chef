# Gzip

Compresses data using the deflate algorithm with gzip headers. Compression type options: Fixed, Dynamic (default), No compression.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Gzip"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Compression type | `Dynamic` | Compression level: Dynamic, Best speed, Best compression, No compression |
| 2 | Filename (optional) | `<empty>` | Optional filename to embed in the gzip header |

