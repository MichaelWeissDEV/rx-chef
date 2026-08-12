# To UNIX Timestamp

Parses a datetime string in UTC and returns the corresponding UNIX timestamp.

e.g. Mon 1 January 2001 11:00:00 becomes 978346800

A UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch).

- Input: `String`
- Output: `String`
- CLI: `rxchef run "To UNIX Timestamp"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Units | `Seconds (s)` | The unit of the timestamp |
| 2 | Treat as UTC | `true` | Treat the input as UTC |
| 3 | Show parsed datetime | `true` | Show the parsed datetime in the output |

