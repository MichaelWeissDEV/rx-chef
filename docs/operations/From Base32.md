# From Base32

Base32 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. It uses a smaller set of characters than Base64, usually the uppercase alphabet and the numbers 2 to 7.

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "From Base32"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Alphabet | `A-Z2-7` | The Base32 alphabet |
| 2 | Remove non-alphabet chars | `true` | Remove characters not in the alphabet before decoding |

