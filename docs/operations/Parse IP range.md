# Parse IP range

Given a CIDR range (e.g. 10.0.0.0/24), a hyphenated range (e.g. 10.0.0.0 - 10.0.1.0), or a single IP address, this operation provides network information and enumerates all IP addresses in the range. IPv6 is supported but not enumerated.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Parse IP range"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Include network info | `true` | Show network/mask/host information |
| 2 | Enumerate IP addresses | `true` | List every IP in the range |
| 3 | Allow large queries | `false` | Allow ranges larger than 65536 |

