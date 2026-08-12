# Group IP addresses

Groups a list of IP addresses into subnets. Supports both IPv4 and IPv6 addresses.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Group IP addresses"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Delimiter | `Line feed` | The delimiter between IP addresses |
| 2 | Subnet (CIDR) | `24` | The CIDR subnet mask to group by |
| 3 | Only show the subnets | `false` | Only show the resulting subnets, not the individual IP addresses |

