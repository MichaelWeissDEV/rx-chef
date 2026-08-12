# Strings

Extracts all strings from the input.

- Input: `Bytes`
- Output: `String`
- CLI: `rxchef run "Strings"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Encoding | `Single byte` | Single byte, 16-bit littleendian, 16-bit bigendian, All |
| 2 | Minimum length | `4` | Minimum string length |
| 3 | Match | `All printable chars (A)` | Alphanumeric + punctuation (A), All printable chars (A), Null-terminated strings (A), Alphanumeric + punctuation (U), All printable chars (U), Null-terminated strings (U) |
| 4 | Display total | `false` | Display total count of found strings |
| 5 | Sort | `false` | Sort results case-insensitively |
| 6 | Unique | `false` | Remove duplicate results |

