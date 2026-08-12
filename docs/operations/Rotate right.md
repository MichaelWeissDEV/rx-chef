# Rotate right

Rotates each byte to the right by the number of bits specified, optionally carrying the excess bits over to the next byte. Currently only supports 8-bit values.

- Input: `Bytes`
- Output: `Bytes`
- CLI: `rxchef run "Rotate right"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Amount | `1` | Number of bits to rotate right |
| 2 | Carry through | `false` | If true, carry bits from one byte to the next across all bytes |

