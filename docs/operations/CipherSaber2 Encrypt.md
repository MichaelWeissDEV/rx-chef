# CipherSaber2 Encrypt

CipherSaber is a simple symmetric encryption protocol based on the RC4 stream cipher. It gives reasonably strong protection of message confidentiality, yet it's designed to be simple enough that even novice programmers can memorize the algorithm and implement it from scratch. A random 10-byte IV is prepended to the ciphertext output.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "CipherSaber2 Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Encryption key |
| 2 | Rounds | `20` | Number of key schedule rounds (default 20) |

