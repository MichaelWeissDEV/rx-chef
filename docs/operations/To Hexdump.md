# To Hexdump

Creates a hexdump of the input data, displaying both the hexadecimal values of each byte and an ASCII representation alongside.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "To Hexdump"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Width | `16` | Number of bytes per row (must be >= 1) |
| 2 | Upper case hex | `false` | Display hex bytes in upper case |
| 3 | Include final length | `false` | Append the total byte count as a final line |
| 4 | UNIX format | `false` | Use UNIX printable character subset for ASCII column |

