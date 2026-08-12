# Triple DES Encrypt

Encrypts data using Triple DES (3DES). Key must be 16 or 24 bytes. IV must be 8 bytes for CBC mode. Modes supported: CBC, ECB. Input/output can be Raw or Hex.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Triple DES Encrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Encryption key (16 or 24 bytes). Encoding: Hex, UTF8, Latin1, Base64 |
| 2 | Key encoding | `Hex` | Encoding of the key: Hex, UTF8, Latin1, Base64 |
| 3 | IV | `<empty>` | Initialization vector (8 bytes for CBC). Encoding: Hex, UTF8, Latin1, Base64 |
| 4 | IV encoding | `Hex` | Encoding of the IV: Hex, UTF8, Latin1, Base64 |
| 5 | Mode | `CBC` | Cipher mode: CBC, ECB |
| 6 | Input | `Raw` | Input encoding: Raw, Hex |
| 7 | Output | `Hex` | Output encoding: Hex, Raw |

