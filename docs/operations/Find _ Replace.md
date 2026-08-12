# Find / Replace

Replaces all occurrences of the first string with the second. Supports regex, simple string, and extended string modes.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Find / Replace"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Find | `<empty>` | The string or regex to find |
| 2 | Find type | `Simple string` | Regex, Extended (\\n, \\t, \\x...), or Simple string |
| 3 | Replace | `<empty>` | The replacement string |
| 4 | Global match | `true` | Replace all occurrences |
| 5 | Case insensitive | `false` | Ignore case when matching |
| 6 | Multiline matching | `true` | ^ and $ match start/end of lines |
| 7 | Dot matches all | `false` | Dot also matches newline |

