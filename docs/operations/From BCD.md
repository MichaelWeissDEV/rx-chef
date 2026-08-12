# From BCD

Binary-Coded Decimal (BCD) is a class of binary encodings of decimal numbers where each decimal digit is represented by a fixed number of bits, usually four or eight. Special bit patterns are sometimes used for a sign.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "From BCD"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Scheme | `8 4 2 1` | The BCD encoding scheme |
| 2 | Packed | `true` | Whether the BCD is packed (two digits per byte) |
| 3 | Signed | `false` | Whether the BCD is signed (trailing sign nibble) |
| 4 | Input format | `Nibbles` | The format of the input data |

