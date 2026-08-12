# ChaCha

ChaCha is a stream cipher designed by Daniel J. Bernstein. It is a variant of the Salsa stream cipher. Several parameterizations exist; 'ChaCha' may refer to the original construction, or to the variant as described in RFC-8439. ChaCha is often used with Poly1305, in the ChaCha20-Poly1305 AEAD construction.<br><br><b>Key:</b> ChaCha uses a key of 16 or 32 bytes (128 or 256 bits).<br><br><b>Nonce:</b> ChaCha uses a nonce of 8 or 12 bytes (64 or 96 bits).<br><br><b>Counter:</b> ChaCha uses a counter of 4 or 8 bytes (32 or 64 bits); together, the nonce and counter must add up to 16 bytes. The counter starts at zero at the start of the keystream, and is incremented at every 64 bytes.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "ChaCha"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The encryption key (16 or 32 bytes) |
| 2 | Nonce | `<empty>` | The nonce (8 or 12 bytes) |
| 3 | Counter | `0` | Initial counter value |
| 4 | Rounds | `20` | Number of rounds (20, 12, or 8) |
| 5 | Input | `Hex` | Format of input data |
| 6 | Output | `Raw` | Format of output data |

