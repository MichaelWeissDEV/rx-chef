# Register

Extract data from the input into recipe registers using regular expression capture groups. Refer to captures in later operation arguments as $R0, $R1, and so on. Register expansion is implemented by integration::bake and all CLI recipe frontends.

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

