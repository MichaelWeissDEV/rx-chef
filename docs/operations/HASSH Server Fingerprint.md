# HASSH Server Fingerprint

Generates a HASSH fingerprint to help identify SSH servers based on hashing together values from the Server Key Exchange Init message.<br><br>Input: A hex stream of the SSH_MSG_KEXINIT packet application layer from Server to Client.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "HASSH Server Fingerprint"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Input format | `Hex` | Input format |
| 2 | Output format | `Hash digest` | Output format |

