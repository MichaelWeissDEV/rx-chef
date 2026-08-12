# Extract domains

Extracts fully qualified domain names.
Note that this will not include paths. Use Extract URLs to find entire URLs.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Extract domains"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Display total | `false` | Display total found |
| 2 | Sort | `false` | Sort results |
| 3 | Unique | `false` | Remove duplicate results |
| 4 | Underscore (DMARC, DKIM, etc) | `false` | Allow underscores in domain labels |

