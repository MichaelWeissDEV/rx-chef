# Extract Files

Carves PNG, JPEG, GIF, PDF, and ZIP signatures from binary input. Matching payloads are returned in a deterministic binary envelope with an ASCII file-type header before each payload.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Extract Files"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Images | `true` | Extract images |
| 2 | Documents | `true` | Extract documents |
| 3 | Archives | `true` | Extract archives |
| 4 | Ignore failed extractions | `true` | Ignore failed extractions |
| 5 | Minimum File Size | `100` | Minimum file size to extract |

