# From Base45

Base45 is a notation for encoding arbitrary byte data using a restricted set of symbols that can be conveniently used by humans and processed by computers. The high number base results in shorter strings than with the decimal or hexadecimal system. Base45 is optimized for usage with QR codes.

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "From Base45"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Alphabet | `0-9A-Z $%*+-./:` | The Base45 alphabet |
| 2 | Remove non-alphabet chars | `true` | Remove characters not in the alphabet before decoding |

