# Triple DES Decrypt

Decrypts data using Triple DES (3DES). Key must be 16 or 24 bytes. IV must be 8 bytes for CBC mode. Modes supported: CBC, ECB. Input/output can be Hex or Raw.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Triple DES Decrypt"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | Decryption key (16 or 24 bytes). Encoding: Hex, UTF8, Latin1, Base64 |
| 2 | Key encoding | `Hex` | Encoding of the key: Hex, UTF8, Latin1, Base64 |
| 3 | IV | `<empty>` | Initialization vector (8 bytes for CBC). Encoding: Hex, UTF8, Latin1, Base64 |
| 4 | IV encoding | `Hex` | Encoding of the IV: Hex, UTF8, Latin1, Base64 |
| 5 | Mode | `CBC` | Cipher mode: CBC, ECB, CBC/NoPadding, ECB/NoPadding |
| 6 | Input | `Hex` | Input encoding: Hex, Raw |
| 7 | Output | `Raw` | Output encoding: Raw, Hex |

