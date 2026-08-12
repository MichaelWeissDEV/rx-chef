# Windows Filetime to UNIX Timestamp

Converts a Windows Filetime value to a UNIX timestamp.<br><br>A Windows Filetime is a 64-bit value representing the number of 100-nanosecond intervals since January 1, 1601 UTC.<br><br>A UNIX timestamp is a 32-bit value representing the number of seconds since January 1, 1970 UTC (the UNIX epoch).<br><br>This operation also supports UNIX timestamps in milliseconds, microseconds and nanoseconds.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Windows Filetime to UNIX Timestamp"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Output units | `Seconds (s)` | Units for the output timestamp |
| 2 | Input format | `Decimal` | Format of the input filetime |

