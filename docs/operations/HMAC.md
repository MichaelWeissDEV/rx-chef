# HMAC

Keyed-Hash Message Authentication Codes (HMAC) are a mechanism for message authentication using cryptographic hash functions.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "HMAC"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | The secret key (Hex, Base64, UTF8, or Latin1) |
| 2 | Hashing function | `SHA-256` | Hashing algorithm (MD5, SHA-1, SHA-256, SHA-384, SHA-512) |
| 3 | Output encoding | `Hex` | Output encoding (Hex, Base64) |

