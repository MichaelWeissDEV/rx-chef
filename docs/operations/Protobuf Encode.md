# Protobuf Encode

Encodes JSON into Protobuf bytes. With a .proto schema, JSON field names and types are resolved from the first top-level message. Without a schema, numeric JSON keys are interpreted as field numbers.

- Input: `JSON`
- Output: `Bytes`
- CLI: `rxchef run "Protobuf Encode"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Schema (.proto text) | `<empty>` | Optional .proto schema; the first top-level message is used |

