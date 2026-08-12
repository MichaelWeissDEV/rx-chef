# Extract URLs

Extracts Uniform Resource Locators (URLs) from the input. The protocol (http, ftp etc.) is required otherwise there will be far too many false positives.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Extract URLs"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Display total | `false` | Display total found |
| 2 | Sort | `false` | Sort results |
| 3 | Unique | `false` | Remove duplicate results |

