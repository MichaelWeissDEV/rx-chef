# Parse DateTime

Parses a DateTime string using strftime format specifiers and displays detailed date/time information including day of year, week number, quarter, and leap year status. Format uses strftime tokens (e.g. %d/%m/%Y %H:%M:%S).

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Parse DateTime"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format string | `%d/%m/%Y %H:%M:%S` | strftime format string (e.g. %d/%m/%Y %H:%M:%S) |
| 2 | Input timezone | `UTC` | Timezone name (currently UTC only) |

