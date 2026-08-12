# To BCD

Binary-Coded Decimal (BCD) is a class of binary encodings of decimal numbers where each decimal digit is represented by a fixed number of bits, usually four or eight. Special bit patterns are sometimes used for a sign.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "To BCD"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Scheme | `8 4 2 1` | The BCD encoding scheme |
| 2 | Packed | `true` | Whether to pack two digits per byte |
| 3 | Signed | `false` | Whether to include a trailing sign nibble |
| 4 | Output format | `Nibbles` | Nibbles, Bytes, or Raw |

