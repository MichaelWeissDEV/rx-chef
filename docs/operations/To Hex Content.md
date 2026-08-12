# To Hex Content

Converts special characters in a string to hexadecimal using SNORT pipe notation. e.g. 'foo=bar' becomes 'foo|3d|bar'.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "To Hex Content"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Convert | `Only special chars` | Only special chars, Only special chars including spaces, or All chars |
| 2 | Print spaces between bytes | `false` | Add spaces between hex bytes inside pipes |

