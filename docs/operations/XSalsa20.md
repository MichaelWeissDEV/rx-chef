# XSalsa20

XSalsa is an extended-nonce Salsa stream cipher designed by Daniel J. Bernstein. It uses a 32-byte key, a 24-byte nonce, and a 64-bit block counter. The standard 20-round cipher and reduced-round XSalsa12 and XSalsa8 variants are supported.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "XSalsa20"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Key to use for encryption/decryption |
| 2 | Nonce | `<empty>` | Nonce to use |
| 3 | Counter | `0` | Starting counter value |
| 4 | Rounds | `20` | Number of rounds |
| 5 | Input | `Raw` | Input format |
| 6 | Output | `Raw` | Output format |

