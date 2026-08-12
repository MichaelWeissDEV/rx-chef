# Head

Like the UNIX head utility. Gets the first n lines. You can select all but the last n lines by entering a negative value for n. The delimiter can be changed so that instead of lines, fields are selected instead.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Head"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Delimiter | `Line feed` | Record delimiter: Line feed, CRLF, Comma, etc. |
| 2 | Number | `10` | Number of lines to take. Negative value = all but last n. |

