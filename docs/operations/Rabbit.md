# Rabbit

Rabbit is a high-speed stream cipher introduced in 2003 and defined in RFC 4503.<br><br>The cipher uses a 128-bit key and an optional 64-bit initialization vector (IV).<br><br>big-endian: based on RFC4503 and RFC3447<br>little-endian: compatible with Crypto++

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Rabbit"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Key | `<empty>` | 128-bit key |
| 2 | IV | `<empty>` | 64-bit IV |
| 3 | Endianness | `Big` | Big or Little |
| 4 | Input | `Raw` | Raw or Hex |
| 5 | Output | `Raw` | Raw or Hex |

