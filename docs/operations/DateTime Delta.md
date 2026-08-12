# DateTime Delta

Calculates a new DateTime value given an input DateTime value and a time difference (delta) from the input DateTime value. Uses strftime format strings.

- Input: `String`
- Output: `HTML`
- CLI: `rxchef run "DateTime Delta"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Built in formats | `Standard date and time` | Common datetime formats |
| 2 | Input format string | `%d/%m/%Y %H:%M:%S` | strftime format string for parsing and formatting (e.g. %d/%m/%Y %H:%M:%S) |
| 3 | Time Operation | `Add` | Whether to add or subtract the delta |
| 4 | Days | `0` | Number of days |
| 5 | Hours | `0` | Number of hours |
| 6 | Minutes | `0` | Number of minutes |
| 7 | Seconds | `0` | Number of seconds |

