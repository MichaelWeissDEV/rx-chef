# JA3S Fingerprint

Generates a JA3S fingerprint to help identify TLS servers based on hashing together values from the Server Hello.<br><br>Input: A hex stream of the TLS Server Hello record application layer.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "JA3S Fingerprint"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format | `Hex` | Input format |
| 2 | Output format | `Hash digest` | Output format |

