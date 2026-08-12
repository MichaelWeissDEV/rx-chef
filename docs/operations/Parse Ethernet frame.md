# Parse Ethernet frame

Parses an Ethernet frame and shows the deduced values: Source and Destination MAC, optional VLAN IDs, and the inner packet payload.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Parse Ethernet frame"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input type | `Hex` | Raw bytes or Hex string |
| 2 | Return type | `Text output` | Text output, Packet data, or Packet data (hex) |

