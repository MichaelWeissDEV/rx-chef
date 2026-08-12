# HASSH Client Fingerprint

Generates a HASSH fingerprint to help identify SSH clients based on hashing together values from the Client Key Exchange Init message.<br><br>Input: A hex stream of the SSH_MSG_KEXINIT packet application layer from Client to Server.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "HASSH Client Fingerprint"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format | `Hex` | Input format |
| 2 | Output format | `Hash digest` | Output format |

