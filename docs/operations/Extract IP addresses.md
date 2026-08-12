# Extract IP addresses

Extracts all IPv4 and IPv6 addresses.

Warning: Given a string 1.2.3.4.5.6.7.8, this will match 1.2.3.4 and 5.6.7.8 so always check the original input!

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Extract IP addresses"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | IPv4 | `true` | Include IPv4 addresses |
| 2 | IPv6 | `false` | Include IPv6 addresses |
| 3 | Remove local IPv4 addresses | `false` | Exclude local/private IPv4 addresses |
| 4 | Display total | `false` | Display the total number of addresses found |
| 5 | Sort | `false` | Sort the results |
| 6 | Unique | `false` | Remove duplicate results |

