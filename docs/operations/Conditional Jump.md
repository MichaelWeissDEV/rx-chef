# Conditional Jump

Conditionally jump forwards or backwards to the specified Label based on whether the data matches the specified regular expression. In this Rust implementation, it acts as a passthrough since flow control requires recipe-level orchestration.

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

