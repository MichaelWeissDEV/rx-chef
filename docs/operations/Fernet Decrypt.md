# Fernet Decrypt

Fernet is a symmetric encryption method which makes sure that the message encrypted cannot be manipulated/read without the key. It uses URL safe encoding for the keys. Fernet uses 128-bit AES in CBC mode and PKCS7 padding, with HMAC using SHA256 for authentication. The IV is created from os.random(). Key: The key must be 32 bytes (256 bits) encoded with Base64.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Fernet Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Base64url-encoded 32-byte Fernet key |

