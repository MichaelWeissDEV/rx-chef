# Register

Extract data from the input and store it in registers which can then be passed into subsequent operations as arguments. Regular expression capture groups are used to select the data to extract.<br><br>To use registers in arguments, refer to them using the notation <code>$Rn</code> where n is the register number, starting at 0.<br><br>In this implementation, acts as a passthrough (flow control requires recipe-level orchestration).

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Register"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Extractor | `([\\s\\S]*)` | Regular expression capture groups |
| 2 | Case insensitive | `true` | Case insensitive matching |
| 3 | Multiline matching | `false` | Multiline matching |
| 4 | Dot matches all | `false` | Dot matches all |

