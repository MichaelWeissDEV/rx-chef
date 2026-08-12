# UNIX Timestamp to Windows Filetime

Converts a UNIX timestamp to a Windows Filetime value.<br><br>A Windows Filetime is a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 UTC.<br><br>A UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch).<br><br>This operation also supports UNIX timestamps in milliseconds, microseconds and nanoseconds.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "UNIX Timestamp to Windows Filetime"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input units | `Seconds (s)` | Input units |
| 2 | Output format | `Decimal` | Output format |

