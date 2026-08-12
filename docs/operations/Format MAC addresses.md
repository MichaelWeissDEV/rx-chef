# Format MAC addresses

Displays given MAC addresses in multiple different formats. Expects addresses separated by newlines, spaces, or commas.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Format MAC addresses"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Output case | `Both` | Both, Upper only, or Lower only |
| 2 | No delimiter | `true` | Output format with no delimiter (e.g. AABBCCDDEEFF) |
| 3 | Dash delimiter | `true` | Output format with dashes (e.g. AA-BB-CC-DD-EE-FF) |
| 4 | Colon delimiter | `true` | Output format with colons (e.g. AA:BB:CC:DD:EE:FF) |
| 5 | Cisco style | `false` | Output Cisco dot notation (e.g. AABB.CCDD.EEFF) |
| 6 | IPv6 interface ID | `false` | Output as IPv6 interface identifier |

