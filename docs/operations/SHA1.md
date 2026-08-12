# SHA1

The SHA (Secure Hash Algorithm) hash functions were designed by the NSA. SHA-1 is the most established of the existing SHA hash functions and it is used in a variety of security applications and protocols. However, SHA-1's collision resistance has been weakening as new attacks are discovered or improved. The message digest algorithm consists, by default, of 80 rounds.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "SHA1"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Rounds | `80` | Number of rounds (minimum 16) |

