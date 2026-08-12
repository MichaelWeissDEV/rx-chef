# Parse ASN.1 hex string

Abstract Syntax Notation One (ASN.1) is a standard and notation that describes rules and structures for representing, encoding, transmitting, and decoding data in telecommunications and computer networking.<br><br>This operation parses arbitrary ASN.1 data (encoded as an hex string: use the 'To Hex' operation if necessary) and presents the resulting tree.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Parse ASN.1 hex string"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Starting index | `0` | Starting index in the byte array |
| 2 | Truncate octet strings longer than | `32` | Truncate octet strings longer than this value |

