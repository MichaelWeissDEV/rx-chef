# Salsa20

Salsa20 is a stream cipher designed by Daniel J. Bernstein and submitted to the eSTREAM project; Salsa20/8 and Salsa20/12 are round-reduced variants. It is closely related to the ChaCha stream cipher.<br><br><b>Key:</b> Salsa20 uses a key of 16 or 32 bytes (128 or 256 bits).<br><br><b>Nonce:</b> Salsa20 uses a nonce of 8 bytes (64 bits).<br><br><b>Counter:</b> Salsa uses a counter of 8 bytes (64 bits). The counter starts at zero at the start of the keystream, and is incremented at every 64 bytes.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Salsa20"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Key (16 or 32 bytes) |
| 2 | Nonce | `<empty>` | Nonce (8 bytes) |
| 3 | Counter | `0` | Initial counter value |
| 4 | Rounds | `20` | Number of rounds (20, 12, or 8) |
| 5 | Input | `Raw` | Input format (Raw, Hex) |
| 6 | Output | `Raw` | Output format (Raw, Hex) |

