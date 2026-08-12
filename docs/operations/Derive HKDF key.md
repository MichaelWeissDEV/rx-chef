# Derive HKDF key

A simple Hashed Message Authenticaton Code (HMAC)-based key derivation function (HKDF), defined in RFC5869.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Derive HKDF key"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Salt | `<empty>` | The salt to use |
| 2 | Info | `<empty>` | The info to use |
| 3 | Hashing function | `SHA256` | The hashing function to use (SHA1, SHA256, SHA384, SHA512) |
| 4 | Extract mode | `with salt` | The extract mode (with salt, no salt, skip) |
| 5 | L (number of output octets) | `16` | The number of output octets |

