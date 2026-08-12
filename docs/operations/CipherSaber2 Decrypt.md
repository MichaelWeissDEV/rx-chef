# CipherSaber2 Decrypt

CipherSaber is a simple symmetric encryption protocol based on the RC4 stream cipher. It gives reasonably strong protection of message confidentiality, yet it's designed to be simple enough that even novice programmers can memorize the algorithm and implement it from scratch. The first 10 bytes of the input are the initialisation vector (IV).

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "CipherSaber2 Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Decryption key |
| 2 | Rounds | `20` | Number of key schedule rounds (default 20) |

