# JA4Server Fingerprint

Generates a JA4Server Fingerprint (JA4S) to help identify TLS servers or sessions based on hashing together values from the Server Hello.<br><br>Input: A hex stream of the TLS or QUIC Server Hello packet application layer.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "JA4Server Fingerprint"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format | `Hex` | Input format |
| 2 | Output format | `JA4S` | Output format |

