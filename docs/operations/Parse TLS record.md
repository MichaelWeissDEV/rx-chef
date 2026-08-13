# Parse TLS record

Parses one or more TLS records into JSON, including content type, protocol version, declared length, truncation state, handshake type and exact payload bytes. Encrypted and handshake-specific payloads remain hex-encoded.

- Input: `Bytes`
- Output: `JSON`
- CLI: `rxchef run "Parse TLS record"`
- Arguments: none

