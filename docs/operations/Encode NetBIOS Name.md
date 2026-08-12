# Encode NetBIOS Name

NetBIOS names as seen across the client interface to NetBIOS are exactly 16 bytes long. Within the NetBIOS-over-TCP protocols, a longer representation is used.<br><br>There are two levels of encoding. The first level maps a NetBIOS name into a domain system name.  The second level maps the domain system name into the 'compressed' representation required for interaction with the domain name system.<br><br>This operation carries out the first level of encoding. See RFC 1001 for full details.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Encode NetBIOS Name"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Offset | `65` | The offset value used for encoding |

