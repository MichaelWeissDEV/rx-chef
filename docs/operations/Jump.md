# Jump

Jump forwards or backwards to the specified Label. In this Rust implementation, it acts as a passthrough since flow control requires recipe-level orchestration.

- Input: `String`
- Output: `String`
- CLI: `rxchef run "Jump"`

## Arguments

| # | Argument | Default | Description |
|---:|---|---|---|
| 1 | Label name | `<empty>` | The name of the label to jump to |
| 2 | Maximum jumps (if jumping backwards) | `10` | The maximum number of times to jump backwards to prevent infinite loops |

