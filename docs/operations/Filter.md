# Filter

Splits up the input using the specified delimiter and then filters each branch based on a regular expression.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Filter"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Delimiter | `Line feed` | Character(s) to split the input on (Line feed, CRLF, Space, Comma, Semi-colon) |
| 2 | Regex | `<empty>` | Regular expression to filter lines |
| 3 | Invert condition | `false` | Return lines that do NOT match the regex |

