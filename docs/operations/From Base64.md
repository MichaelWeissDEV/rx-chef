# From Base64

Base64 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. This operation decodes data from an ASCII Base64 string back into its raw format.

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "From Base64"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Alphabet | `A-Za-z0-9+/=` | The Base64 alphabet |
| 2 | Remove non-alphabet chars | `true` | Remove characters not in the alphabet before decoding |
| 3 | Strict mode | `false` | Throw an error if the input is not perfectly formatted |

