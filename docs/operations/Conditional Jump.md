# Conditional Jump

Conditionally jump forwards or backwards to a Label when the current data matches a regular expression. Backwards jumps are bounded by the configured maximum. Interpreted by integration::bake and all CLI recipe frontends.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Conditional Jump"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Match (regex) | `<empty>` | The regular expression to match against the data |
| 2 | Invert match | `false` | If true, jump when the regex does NOT match |
| 3 | Label name | `<empty>` | The name of the label to jump to |
| 4 | Maximum jumps (if jumping backwards) | `10` | The maximum number of times to jump backwards to prevent infinite loops |

