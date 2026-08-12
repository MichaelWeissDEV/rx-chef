# Extract file paths

Extracts anything that looks like a Windows or UNIX file path.

Note that if UNIX is selected, there will likely be a lot of false positives.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Extract file paths"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Windows | `true` | Include Windows file paths |
| 2 | UNIX | `true` | Include UNIX file paths |
| 3 | Display total | `false` | Display the total number of paths found |
| 4 | Sort | `false` | Sort the results |
| 5 | Unique | `false` | Remove duplicate results |

