# Protobuf Decode

Decodes Protobuf bytes to JSON. With a .proto schema, field names and declared types are used. Without a schema, wire fields are represented by numeric keys.

- Input: `Bytes`
- Output: `JSON`
- CLI: `rxchef run "Protobuf Decode"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Schema (.proto text) | `<empty>` | Optional .proto schema; the first top-level message is used |
| 2 | Show Unknown Fields | `false` | Show fields not in schema |
| 3 | Show Types | `false` | Show type information |

