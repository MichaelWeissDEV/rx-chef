# Parse TLV

Converts a Type-Length-Value (TLV) encoded string into a JSON object. Can optionally include a Key / Type entry.

- Input: `Bytes`
- Output: `JSON`
- CLI: `rxchef run "Parse TLV"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Type/Key size | `1` | Size of the type/key field in bytes |
| 2 | Length size | `1` | Size of the length field in bytes |
| 3 | Use BER | `false` | Use Basic Encoding Rules for length field |

