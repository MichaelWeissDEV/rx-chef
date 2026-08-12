# From Base85

Base85 (also called Ascii85) is a notation for encoding arbitrary byte data. It is usually more efficient than Base64.

This operation decodes data from an ASCII string (with an alphabet of your choosing, presets included).

e.g. BOu!rD]j7BEbo7 becomes hello world

Base85 is commonly used in Adobe's PostScript and PDF file formats.

- Input: `String`
- Output: `Bytes`
- CLI: `rxchef run "From Base85"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Alphabet | `!-u` | The Base85 alphabet |
| 2 | Remove non-alphabet chars | `true` | Remove characters not in the alphabet before decoding |
| 3 | All-zero group char | `z` | Character representing an all-zero group (default 'z') |

