# To Base85

Base85 (also called Ascii85) is a notation for encoding arbitrary byte data. It is usually more efficient than Base64.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "To Base85"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Alphabet | `!-u` | The Base85 alphabet |
| 2 | Include delimiter | `false` | Adds a <~ and ~> delimiter to the start and end of the data. |

