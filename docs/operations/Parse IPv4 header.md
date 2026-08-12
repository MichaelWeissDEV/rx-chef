# Parse IPv4 header

Parses an IPv4 packet header and displays each field in a readable format including version, IHL, DSCP, ECN, total length, identification, flags, fragment offset, TTL, protocol, checksum, source and destination IP addresses, and options if present.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Parse IPv4 header"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format | `Hex` | Hex or Raw |
| 2 | Output format | `Table` | Table, Data (hex), or Data (raw) |

