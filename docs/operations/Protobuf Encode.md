# Protobuf Encode

Encodes a valid JSON object into a protobuf byte array. Note: This implementation currently only supports encoding based on numeric keys in the JSON input (field numbers) as runtime schema compilation is not supported.

- Input: `JSON`
- Output: `Bytes`
- CLI: `rxchef run "Protobuf Encode"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Schema (.proto text) | `<empty>` | Optional schema (not implemented in this version) |

