# Protobuf Decode

Decodes any Protobuf encoded data to a JSON representation of the data using the field number as the field key.

- Input: `Bytes`
- Output: `JSON`
- CLI: `rxchef run "Protobuf Decode"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Schema (.proto text) | `<empty>` | Optional schema (not implemented in this version) |
| 2 | Show Unknown Fields | `false` | Show fields not in schema |
| 3 | Show Types | `false` | Show type information |

