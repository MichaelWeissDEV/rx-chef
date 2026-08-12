# XSalsa20

XSalsa20 is a variant of the Salsa20 stream cipher designed by Daniel J. Bernstein; XSalsa uses longer nonces.<br><br><b>Key:</b> XSalsa20 uses a key of 16 or 32 bytes (128 or 256 bits).<br><br><b>Nonce:</b> XSalsa20 uses a nonce of 24 bytes (192 bits).<br><br><b>Counter:</b> XSalsa uses a counter of 8 bytes (64 bits). The counter starts at zero at the start of the keystream, and is incremented at every 64 bytes.

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

