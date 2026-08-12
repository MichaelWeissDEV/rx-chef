# JA3 Fingerprint

Generates a JA3 fingerprint to help identify TLS clients based on hashing together values from the Client Hello.<br><br>Input: A hex stream of the TLS Client Hello packet application layer.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "JA3 Fingerprint"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format | `Hex` | Input format |
| 2 | Output format | `Hash digest` | Output format |

