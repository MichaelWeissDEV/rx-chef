# From Base58

Base58 (similar to Base64) is a notation for encoding arbitrary byte data. It differs from Base64 by removing easily misread characters (i.e. l, I, 0 and O) to improve human readability. This operation decodes data from an ASCII string back into its raw form.

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "From Base58"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Alphabet | `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz` | The Base58 alphabet |
| 2 | Remove non-alphabet chars | `true` | Remove characters not in the alphabet before decoding |

