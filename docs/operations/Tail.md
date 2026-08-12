# Tail

Like the UNIX tail utility. Gets the last n lines. Optionally you can select all lines after line n by entering a negative value for n. The delimiter can be changed so that instead of lines, fields are selected instead.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Tail"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Delimiter | `Line feed` | Record delimiter: Line feed, CRLF, Comma, etc. |
| 2 | Number | `10` | Number of lines to take from the end. Negative = all after line n. |

