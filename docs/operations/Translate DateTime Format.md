# Translate DateTime Format

Parses a datetime string in one format and re-writes it in another. Uses strftime/strptime format strings (e.g. %d/%m/%Y %H:%M:%S). Timezone names are noted but conversion uses UTC unless a numeric offset is embedded in the format.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Translate DateTime Format"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format string | `%d/%m/%Y %H:%M:%S` | strftime format string for parsing input (e.g. %d/%m/%Y %H:%M:%S) |
| 2 | Input timezone | `UTC` | Timezone of the input datetime (informational; UTC assumed unless offset in format) |
| 3 | Output format string | `%A %d %B %Y %H:%M:%S` | strftime format string for the output (e.g. %A %d %B %Y %H:%M:%S %z) |
| 4 | Output timezone | `UTC` | Timezone for the output datetime (informational; UTC assumed unless offset in format) |

