# IPv6 Transition Addresses

Converts IPv4 addresses to their IPv6 Transition addresses. IPv6 Transition addresses can also be converted back into their original IPv4 address. MAC addresses can also be converted into the EUI-64 format, this can them be appended to your IPv6 /64 range to obtain a full /128 address.<br><br>Transition technologies enable translation between IPv4 and IPv6 addresses or tunneling to allow traffic to pass through the incompatible network, allowing the two standards to coexist.<br><br>Only /24 ranges and currently handled. Remove headers to easily copy out results.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "IPv6 Transition Addresses"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Ignore ranges | `true` | If checked, input ranges will be ignored. |
| 2 | Remove headers | `false` | Remove headers to easily copy out results. |

