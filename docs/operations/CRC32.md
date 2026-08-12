# CRC32

CRC32 (Cyclic Redundancy Check) is a hash function that produces a 32-bit checksum. It is widely used for error detection in digital networks and storage devices.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "CRC32"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Polynomial | `IEEE` | CRC polynomial (default: IEEE) |
| 2 | Initial Value | `0xFFFFFFFF` | Initial CRC value (default: 0xFFFFFFFF) |
| 3 | Reflect Input | `true` | Reflect input bytes (true/false) |
| 4 | Reflect Output | `true` | Reflect output CRC (true/false) |
| 5 | XOR Output | `0xFFFFFFFF` | XOR output with this value (default: 0xFFFFFFFF) |

